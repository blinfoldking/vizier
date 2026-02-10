use anyhow::Result;
use rig::agent::Agent;
use rig::client::{CompletionClient, Nothing};
use rig::completion::{Chat, CompletionModel};
use rig::providers::{ollama, openrouter};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::agent::memory::SessionMemory;
use crate::agent::session::VizierSession;
use crate::config::{AgentConfig, AgentConfigs, MemoryConfig};
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
}

impl VizierAgents {
    pub fn new(
        config: AgentConfigs,
        memory_config: MemoryConfig,
        transport: VizierTransport,
    ) -> Result<Self> {
        Ok(Self {
            config: config,
            memory_config: memory_config,
            transport,
            sessions: HashMap::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let transport = self.transport.clone();
        loop {
            if let Ok((session, request)) = transport.request_reader.recv() {
                // start thinking every 5 second until response ready
                let thinking_transport = transport.clone();
                let thinking_session = session.clone();
                let thinking = tokio::spawn(async move {
                    loop {
                        let _ = thinking_transport
                            .response_writer
                            .send((thinking_session.clone(), VizierResponse::StartThinking));

                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                });

                let content = self.handle_request(&session.clone(), &request).await;
                match content {
                    Err(err) => {
                        log::error!("{}", err);
                    }
                    Ok(content) => {
                        if let Err(err) = transport
                            .response_writer
                            .send((session.clone(), VizierResponse::Message(content)))
                        {
                            log::error!("{}", err);
                        }
                    }
                }

                // stop thinking
                thinking.abort();
                let _ = transport
                    .response_writer
                    .send((session.clone(), VizierResponse::StopThinking));
            }
        }
    }

    pub async fn handle_request(
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

    fn init_session(&mut self) -> Result<VizierAgent> {
        // TODO: hardcord to use primary only for now
        let primary_config = &self.config.primary;
        let agent = match &*primary_config.model.provider.clone() {
            "openrouter" => {
                VizierAgent::OpenRouter(VizierAgentImpl::<openrouter::CompletionModel>::new(
                    primary_config.name.clone(),
                    primary_config,
                    &self.memory_config,
                )?)
            }
            _ => VizierAgent::Ollama(VizierAgentImpl::<ollama::CompletionModel>::new(
                primary_config.name.clone(),
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
}

impl VizierAgent {
    pub async fn handle_chat(&mut self, req: VizierRequest) -> Result<String> {
        let response = match self {
            Self::Ollama(agent) => agent.chat(req).await,
            Self::OpenRouter(agent) => agent.chat(req).await,
        }?;

        Ok(response.to_string())
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
                format!("{}\n{}", req.content, self.session_memory.summary_prompt()),
                self.session_memory.recall_as_messages(),
            )
            .await?;

        let response_msg = remove_think_tags(&*response.to_string());
        self.session_memory
            .push_agent(self.agent.name.clone().unwrap(), response_msg);

        Ok(response.to_string())
    }
}

impl VizierAgentImpl<ollama::CompletionModel> {
    fn new(id: String, config: &AgentConfig, memory_config: &MemoryConfig) -> Result<Self> {
        let client: ollama::Client = ollama::Client::builder()
            .base_url(config.model.base_url.clone())
            .api_key(Nothing)
            .build()?;

        let agent = client
            .agent(config.model.name.clone())
            .name(&*config.model.name.clone())
            .preamble(&config.preamble)
            .build();

        Ok(Self {
            id: id.clone(),
            agent,
            session_memory: SessionMemory::new(memory_config.clone()),
        })
    }
}

impl VizierAgentImpl<openrouter::CompletionModel> {
    fn new(id: String, config: &AgentConfig, memory_config: &MemoryConfig) -> Result<Self> {
        let client: openrouter::Client = openrouter::Client::new(config.model.api_key.clone())?;

        let agent = client
            .agent(config.model.name.clone())
            .name(&*config.model.name.clone())
            .preamble(&config.preamble)
            .build();

        Ok(Self {
            id: id.clone(),
            agent,
            session_memory: SessionMemory::new(memory_config.clone()),
        })
    }
}
