use anyhow::Result;
use axum::http::request;
use rig::agent::Agent;
use rig::client::{CompletionClient, Nothing};
use rig::completion::{Chat, CompletionModel, Prompt};
use rig::providers::{deepseek, ollama, openrouter};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::agent::memory::SessionMemory;
use crate::agent::session::VizierSession;
use crate::agent::tools::VizierTools;
use crate::config::{AgentConfig, AgentConfigs, MemoryConfig, ToolsConfig};
use crate::transport::{VizierRequest, VizierResponse, VizierTransport};
use crate::utils::remove_think_tags;

pub mod memory;
pub mod session;
pub mod tools;

#[derive(Clone)]
pub struct VizierAgents {
    config: AgentConfigs,
    memory_config: MemoryConfig,
    transport: VizierTransport,
    sessions: HashMap<VizierSession, Arc<Mutex<VizierAgent>>>,
    tools: VizierTools,
}

impl VizierAgents {
    pub async fn new(
        workspace: String,
        config: AgentConfigs,
        memory_config: MemoryConfig,
        tool_config: ToolsConfig,
        transport: VizierTransport,
    ) -> Result<Self> {
        Ok(Self {
            config: config,
            memory_config: memory_config,
            transport,
            tools: VizierTools::new(workspace.clone(), tool_config).await,
            sessions: HashMap::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let transport = self.transport.clone();
        loop {
            if let Ok((session, request)) = transport.read_request().await {
                // handle user requested lobotomy
                let lobotomy_transport = self.transport.clone();
                if request.content == "/lobotomy" {
                    let _ = self.handle_lobotomy(&session).await;
                    tokio::spawn(async move {
                        if let Err(err) = lobotomy_transport
                            .send_response(
                                session.clone(),
                                VizierResponse::Message("YIPEEEE".into()),
                            )
                            .await
                        {
                            log::error!("{}", err);
                        }
                    });

                    continue;
                }

                if request.is_silent_read {
                    self.handle_silent_read(&session, &request).await?;
                    continue;
                }

                // start thinking every 5 second until response ready
                let thinking_transport = transport.clone();
                let thinking_session = session.clone();
                let thinking = tokio::spawn(async move {
                    loop {
                        let _ = thinking_transport
                            .send_response(thinking_session.clone(), VizierResponse::Thinking)
                            .await;

                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                });

                let content = self.handle_chat(&session.clone(), &request).await;
                match content {
                    Err(err) => {
                        log::error!("{}", err);
                    }
                    Ok(content) => {
                        if let Err(err) = transport
                            .send_response(session.clone(), VizierResponse::Message(content))
                            .await
                        {
                            log::error!("{}", err);
                        }
                    }
                }

                // stop thinking
                thinking.abort();
            }
        }
    }

    pub async fn handle_silent_read(
        &mut self,
        session: &VizierSession,
        req: &VizierRequest,
    ) -> Result<()> {
        // find session, if none found, make it then retry
        if let Some(agent) = self.sessions.get(session) {
            agent.lock().await.handle_silent_read(req.clone()).await?
        } else {
            let agent = Arc::new(Mutex::new(self.init_session()?));
            self.sessions.insert(session.clone(), agent.clone());
            agent.lock().await.handle_silent_read(req.clone()).await?;
        }

        Ok(())
    }

    pub async fn handle_chat(
        &mut self,
        session: &VizierSession,
        req: &VizierRequest,
    ) -> Result<String> {
        // find session, if none found, make it then retry
        if let Some(agent) = self.sessions.get(session) {
            agent.lock().await.handle_chat(req.clone()).await
        } else {
            let agent = Arc::new(Mutex::new(self.init_session()?));
            self.sessions.insert(session.clone(), agent.clone());
            agent.lock().await.handle_chat(req.clone()).await
        }
    }

    pub async fn handle_lobotomy(&mut self, session: &VizierSession) -> Result<()> {
        // find session, if none found, make it then retry
        if let Some(agent) = self.sessions.get(session) {
            agent.lock().await.handle_lobotomy().await?;
        }

        Ok(())
    }

    fn init_session(&mut self) -> Result<VizierAgent> {
        // TODO: hardcord to use primary only for now
        let primary_config = &self.config.primary;
        let agent = match &*primary_config.model.provider.clone() {
            "openrouter" => {
                VizierAgent::OpenRouter(VizierAgentImpl::<openrouter::CompletionModel>::new(
                    primary_config.name.clone(),
                    self.tools.clone(),
                    primary_config,
                    &self.memory_config,
                )?)
            }
            "deepseek" => VizierAgent::Deepseek(VizierAgentImpl::<deepseek::CompletionModel>::new(
                primary_config.name.clone(),
                self.tools.clone(),
                primary_config,
                &self.memory_config,
            )?),
            _ => VizierAgent::Ollama(VizierAgentImpl::<ollama::CompletionModel>::new(
                primary_config.name.clone(),
                self.tools.clone(),
                primary_config,
                &self.memory_config,
            )?),
        };

        Ok(agent)
    }
}

#[derive(Clone)]
pub enum VizierAgent {
    Ollama(VizierAgentImpl<ollama::CompletionModel>),
    OpenRouter(VizierAgentImpl<openrouter::CompletionModel>),
    Deepseek(VizierAgentImpl<deepseek::CompletionModel>),
}

impl VizierAgent {
    pub async fn handle_chat(&mut self, req: VizierRequest) -> Result<String> {
        let response = match self {
            Self::Ollama(agent) => agent.chat(req).await,
            Self::OpenRouter(agent) => agent.chat(req).await,
            Self::Deepseek(agent) => agent.chat(req).await,
        }?;

        Ok(response.to_string())
    }

    pub async fn handle_silent_read(&mut self, req: VizierRequest) -> Result<()> {
        let _ = match self {
            Self::Ollama(agent) => agent.silent_read(req).await,
            Self::OpenRouter(agent) => agent.silent_read(req).await,
            Self::Deepseek(agent) => agent.silent_read(req).await,
        }?;

        Ok(())
    }

    pub async fn handle_lobotomy(&mut self) -> Result<()> {
        let _ = match self {
            Self::Ollama(agent) => agent.lobotomy().await,
            Self::OpenRouter(agent) => agent.lobotomy().await,
            Self::Deepseek(agent) => agent.lobotomy().await,
        };

        Ok(())
    }
}

#[derive(Clone)]
pub struct VizierAgentImpl<T: CompletionModel> {
    id: String,
    agent: Agent<T>,
    session_memory: SessionMemory,
}

impl<T: CompletionModel> VizierAgentImpl<T> {
    async fn chat(&mut self, req: VizierRequest) -> Result<String> {
        self.session_memory.push_user_message(req.clone());
        let response = self
            .agent
            .chat(
                format!(
                    "{}\n\n{}",
                    req.to_prompt()?,
                    self.session_memory.summary_prompt()
                ),
                self.session_memory.recall_as_messages(),
            )
            .await?;

        let response_msg = remove_think_tags(&*response.to_string());
        self.session_memory.push_agent(response_msg);

        self.session_memory.try_summarize(self.clone()).await?;

        Ok(response.to_string())
    }

    async fn silent_read(&mut self, req: VizierRequest) -> Result<()> {
        self.session_memory.push_user_message(req.clone());

        Ok(())
    }

    async fn lobotomy(&mut self) {
        self.session_memory.flush();
    }
}

impl VizierAgentImpl<ollama::CompletionModel> {
    fn new(
        id: String,
        tool: VizierTools,
        config: &AgentConfig,
        memory_config: &MemoryConfig,
    ) -> Result<Self> {
        let client: ollama::Client = ollama::Client::builder()
            .base_url(config.model.base_url.clone())
            .api_key(Nothing)
            .build()?;

        let boot = std::fs::read_to_string(std::path::PathBuf::from(format!(
            "{}/BOOT.md",
            tool.workspace
        )))?;

        let agent = client
            .agent(config.model.name.clone())
            .name(&*config.model.name.clone())
            .preamble(&boot)
            .tool_server_handle(tool.handle)
            .default_max_turns(tool.turn_depth as usize)
            .build();

        Ok(Self {
            id: id.clone(),
            agent,
            session_memory: SessionMemory::new(memory_config.clone()),
        })
    }
}

impl VizierAgentImpl<openrouter::CompletionModel> {
    fn new(
        id: String,
        tool: VizierTools,
        config: &AgentConfig,
        memory_config: &MemoryConfig,
    ) -> Result<Self> {
        let client: openrouter::Client = openrouter::Client::new(config.model.api_key.clone())?;

        let boot = std::fs::read_to_string(std::path::PathBuf::from(format!(
            "{}/BOOT.md",
            tool.workspace
        )))?;

        let agent = client
            .agent(config.model.name.clone())
            .name(&*config.model.name.clone())
            .preamble(&boot)
            .tool_server_handle(tool.handle)
            .default_max_turns(tool.turn_depth as usize)
            .build();

        Ok(Self {
            id: id.clone(),
            agent,
            session_memory: SessionMemory::new(memory_config.clone()),
        })
    }
}

impl VizierAgentImpl<deepseek::CompletionModel> {
    fn new(
        id: String,
        tool: VizierTools,
        config: &AgentConfig,
        memory_config: &MemoryConfig,
    ) -> Result<Self> {
        let client: deepseek::Client = deepseek::Client::new(config.model.api_key.clone())?;

        let boot = std::fs::read_to_string(std::path::PathBuf::from(format!(
            "{}/BOOT.md",
            tool.workspace
        )))?;

        let agent = client
            .agent(config.model.name.clone())
            .name(&*config.model.name.clone())
            .preamble(&boot)
            .tool_server_handle(tool.handle)
            .default_max_turns(tool.turn_depth as usize)
            .build();

        Ok(Self {
            id: id.clone(),
            agent,
            session_memory: SessionMemory::new(memory_config.clone()),
        })
    }
}
