use anyhow::Result;
use chrono::Utc;
use rig::agent::Agent;
use rig::client::{CompletionClient, Nothing};
use rig::completion::{Chat, CompletionModel};
use rig::message::Message;
use rig::providers::{deepseek, ollama, openrouter};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::agent::memory::SessionMemories;
use crate::agent::session::VizierSession;
use crate::agent::tools::VizierTools;
use crate::config::ToolsConfig;
use crate::config::provider::ProviderVariant;
use crate::dependencies::VizierDependencies;
use crate::transport::{VizierRequest, VizierResponse};
use crate::utils::{agent_workspace, init_workspace, remove_think_tags};

pub mod exec;
pub mod memory;
pub mod session;
pub mod tools;

#[derive(Clone)]
pub struct VizierAgents {
    deps: VizierDependencies,
    sessions: Arc<Mutex<HashMap<VizierSession, VizierAgent>>>,
}

fn read_md_file(workspace: String, file: String) -> String {
    let path = PathBuf::from(format!("{}/{}", workspace, file));

    fs::read_to_string(path).unwrap()
}

impl VizierAgents {
    pub async fn new(deps: VizierDependencies) -> Result<Self> {
        Ok(Self {
            deps,
            sessions: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let sessions = self.sessions.clone();

        let cleanup_sessions = self.sessions.clone();
        // stale agent session killer
        let cleanup_handle = tokio::spawn(async move {
            loop {
                let lookup = cleanup_sessions.lock().await.clone();
                for (session, agent) in lookup.iter() {
                    if agent.is_stale().await {
                        sessions.lock().await.remove(session);
                    }
                }
            }
        });

        let transport = self.deps.transport.clone();
        while let Ok((session, request)) = transport.read_request().await {
            // handle user requested lobotomy
            let lobotomy_transport = self.deps.transport.clone();
            if request.content == "/lobotomy" {
                let _ = self.handle_lobotomy(&session).await;
                tokio::spawn(async move {
                    if let Err(err) = lobotomy_transport
                        .send_response(session.clone(), VizierResponse::Message("YIPEEEE".into()))
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

        cleanup_handle.abort();
        Ok(())
    }

    pub async fn handle_silent_read(
        &mut self,
        session: &VizierSession,
        req: &VizierRequest,
    ) -> Result<()> {
        // find session, if none found, make it then retry
        if let Some(agent) = self.sessions.lock().await.get_mut(session) {
            agent.handle_silent_read(req.clone()).await?
        } else {
            let mut agent = self.init_session(session.0.clone())?;
            agent.handle_silent_read(req.clone()).await?;
            self.sessions
                .lock()
                .await
                .insert(session.clone(), agent.clone());
        }

        Ok(())
    }

    pub async fn handle_chat(
        &mut self,
        session: &VizierSession,
        req: &VizierRequest,
    ) -> Result<String> {
        // find session, if none found, make it then retry
        if let Some(agent) = self.sessions.lock().await.get_mut(session) {
            agent.handle_chat(req.clone()).await
        } else {
            let mut agent = self.init_session(session.0.clone())?;
            let response = agent.handle_chat(req.clone()).await?;
            self.sessions
                .lock()
                .await
                .insert(session.clone(), agent.clone());

            Ok(response)
        }
    }

    pub async fn handle_lobotomy(&mut self, session: &VizierSession) -> Result<()> {
        // find session, if none found, make it then retry
        if let Some(agent) = self.sessions.lock().await.get_mut(session) {
            agent.handle_lobotomy().await?;
        }

        Ok(())
    }

    fn init_session(&mut self, id: String) -> Result<VizierAgent> {
        println!("{id}");
        // TODO: hardcord to use primary only for now
        let agent_config = &self.deps.config.agents.get(&id).unwrap();
        let agent = match &agent_config.provider {
            ProviderVariant::openrouter => {
                VizierAgent::OpenRouter(VizierAgentImpl::<openrouter::CompletionModel>::new(
                    id.clone(),
                    self.deps.clone(),
                )?)
            }

            ProviderVariant::deepseek => VizierAgent::Deepseek(VizierAgentImpl::<
                deepseek::CompletionModel,
            >::new(
                id.clone(), self.deps.clone()
            )?),

            ProviderVariant::ollama => VizierAgent::Ollama(VizierAgentImpl::<
                ollama::CompletionModel,
            >::new(
                id.clone(), self.deps.clone()
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

    pub async fn is_stale(&self) -> bool {
        let res = match self {
            Self::Ollama(agent) => agent.is_stale().await,
            Self::OpenRouter(agent) => agent.is_stale().await,
            Self::Deepseek(agent) => agent.is_stale().await,
        };

        res
    }
}

#[derive(Clone)]
pub struct VizierAgentImpl<T: CompletionModel> {
    #[allow(unused)]
    id: String,
    agent: Agent<T>,
    session_memory: SessionMemories,
    session_ttl: Duration,
    last_interact_at: chrono::DateTime<Utc>,
    workspace: String,
}

impl<T: CompletionModel> VizierAgentImpl<T> {
    async fn chat(&mut self, req: VizierRequest) -> Result<String> {
        let agent_workspace = agent_workspace(&self.workspace, &self.id);

        let agent_md = read_md_file(agent_workspace.clone(), "AGENT.md".into());
        let ident_md = read_md_file(agent_workspace.clone(), "IDENT.md".into());
        let user_md = read_md_file(agent_workspace.clone(), "USER.md".into());

        let mut history = vec![
            Message::user(agent_md),
            Message::user(ident_md),
            Message::user(user_md),
        ];

        history.extend(self.session_memory.recall_as_messages());

        let response = self
            .agent
            .chat(format!("{}", req.to_prompt()?,), history)
            .await?;

        let response_msg = remove_think_tags(&*response.to_string());

        self.session_memory.push_user_message(req.clone());
        self.session_memory.push_agent(response_msg);
        self.session_memory.try_summarize(self.clone()).await?;

        self.last_interact_at = Utc::now();
        Ok(response.to_string())
    }

    async fn silent_read(&mut self, req: VizierRequest) -> Result<()> {
        self.session_memory.push_user_message(req.clone());
        self.session_memory.try_summarize(self.clone()).await?;

        self.last_interact_at = Utc::now();
        Ok(())
    }

    async fn lobotomy(&mut self) {
        self.last_interact_at = Utc::now();
        self.session_memory.flush();
    }

    async fn is_stale(&self) -> bool {
        let diff = Utc::now() - self.last_interact_at;

        diff.to_std().unwrap() > self.session_ttl
    }
}

impl VizierAgentImpl<ollama::CompletionModel> {
    fn new(id: String, deps: VizierDependencies) -> Result<Self> {
        let agent_workspace = agent_workspace(&deps.config.workspace, &id);
        init_workspace(agent_workspace.clone());

        let agent_config = deps.config.agents.get(&id).unwrap();

        let client: ollama::Client = ollama::Client::builder()
            .base_url(deps.config.providers.ollama.clone().unwrap().base_url)
            .api_key(Nothing)
            .build()?;

        let boot = std::fs::read_to_string(std::path::PathBuf::from(format!(
            "{}/BOOT.md",
            agent_workspace
        )))?;

        let tool = VizierTools::new(id.clone(), deps.clone())?;

        let agent = client
            .agent(agent_config.model.clone())
            .name(&agent_config.name.clone())
            .preamble(&boot)
            .tool_server_handle(tool.handle)
            .default_max_turns(agent_config.turn_depth)
            .build();

        Ok(Self {
            id: id.clone(),
            agent,
            session_memory: SessionMemories::new(agent_config.memory.clone()),
            session_ttl: *agent_config.session_ttl,
            last_interact_at: Utc::now(),
            workspace: deps.config.workspace.clone(),
        })
    }
}

impl VizierAgentImpl<openrouter::CompletionModel> {
    fn new(id: String, deps: VizierDependencies) -> Result<Self> {
        let agent_workspace = agent_workspace(&deps.config.workspace, &id);
        init_workspace(agent_workspace.clone());

        let agent_config = deps.config.agents.get(&id).unwrap();

        let client: openrouter::Client =
            openrouter::Client::new(deps.config.providers.openrouter.clone().unwrap().api_key)?;

        let boot = std::fs::read_to_string(std::path::PathBuf::from(format!(
            "{}/BOOT.md",
            agent_workspace
        )))?;

        let tool = VizierTools::new(id.clone(), deps.clone())?;

        let agent = client
            .agent(agent_config.model.clone())
            .name(&agent_config.name)
            .preamble(&boot)
            .tool_server_handle(tool.handle)
            .default_max_turns(agent_config.turn_depth)
            .build();

        Ok(Self {
            id: id.clone(),
            agent,
            session_memory: SessionMemories::new(agent_config.memory.clone()),
            session_ttl: *agent_config.session_ttl,
            last_interact_at: Utc::now(),
            workspace: deps.config.workspace.clone(),
        })
    }
}

impl VizierAgentImpl<deepseek::CompletionModel> {
    fn new(id: String, deps: VizierDependencies) -> Result<Self> {
        let agent_workspace = agent_workspace(&deps.config.workspace, &id);
        init_workspace(agent_workspace.clone());

        let agent_config = deps.config.agents.get(&id).unwrap();

        let client: deepseek::Client =
            deepseek::Client::new(deps.config.providers.deepseek.clone().unwrap().api_key)?;

        let boot = std::fs::read_to_string(std::path::PathBuf::from(format!(
            "{}/BOOT.md",
            agent_workspace
        )))?;

        let tool = VizierTools::new(id.clone(), deps.clone())?;

        let agent = client
            .agent(agent_config.model.clone())
            .name(&agent_config.name)
            .preamble(&boot)
            .tool_server_handle(tool.handle)
            .default_max_turns(agent_config.turn_depth)
            .build();

        Ok(Self {
            id: id.clone(),
            agent,
            session_memory: SessionMemories::new(agent_config.memory.clone()),
            session_ttl: *agent_config.session_ttl,
            last_interact_at: Utc::now(),
            workspace: deps.config.workspace.clone(),
        })
    }
}
