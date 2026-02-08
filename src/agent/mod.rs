use anyhow::Result;
use rig::OneOrMany;
use rig::agent::Agent;
use rig::client::{CompletionClient, Nothing};
use rig::completion::{Chat, CompletionModel, Prompt};
use rig::message::{Message, UserContent};
use rig::providers::{ollama, openrouter};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

use crate::agent::session::VizierSession;
use crate::config::{AgentConfig, AgentConfigs};
use crate::transport::{VizierRequest, VizierResponse, VizierTransport};

pub mod memory;
pub mod session;

#[derive(Clone)]
pub struct VizierAgents {
    config: AgentConfigs,
    transport: VizierTransport,
    sessions: HashMap<VizierSession, Arc<VizierAgent>>,
}

impl VizierAgents {
    pub fn new(config: AgentConfigs, transport: VizierTransport) -> Result<Self> {
        Ok(Self {
            config: config,
            transport,
            sessions: HashMap::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let transport = self.transport.clone();
        loop {
            if let Ok((session, request)) = transport.request_reader.recv() {
                // start thinking
                let _ = transport
                    .response_writer
                    .send((session.clone(), VizierResponse::StartThinking));

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
            agent.handle_prompt(req.content.clone()).await
        } else {
            let agent = Arc::new(self.init_session()?);
            self.sessions.insert(session.clone(), agent.clone());
            agent.handle_prompt(req.content.clone()).await
        }
    }

    fn init_session(&mut self) -> Result<VizierAgent> {
        // TODO: hardcord to use primary only for now
        let primary_config = self.config.get("primary").unwrap();
        let agent = match &*primary_config.model.provider.clone() {
            "openrouter" => {
                VizierAgent::OpenRouter(VizierAgentImpl::<openrouter::CompletionModel>::new(
                    "primary".into(),
                    primary_config,
                )?)
            }
            _ => VizierAgent::Ollama(VizierAgentImpl::<ollama::CompletionModel>::new(
                "primary".into(),
                primary_config,
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
    pub async fn handle_prompt(&self, prompt: String) -> Result<String> {
        let response = match self {
            Self::Ollama(agent) => agent.agent.prompt(prompt).await,
            Self::OpenRouter(agent) => agent.agent.prompt(prompt).await,
        }?;

        Ok(response.to_string())
    }

    pub async fn handle_chat(&self, prompt: String) -> Result<String> {
        let response = match self {
            Self::Ollama(agent) => agent.agent.chat(prompt, vec![]).await,
            Self::OpenRouter(agent) => agent.agent.chat(prompt, vec![]).await,
        }?;

        Ok(response.to_string())
    }
}

#[derive(Clone)]
pub struct VizierAgentImpl<T: CompletionModel> {
    id: String,
    agent: Agent<T>,
}

impl VizierAgentImpl<ollama::CompletionModel> {
    fn new(id: String, config: &AgentConfig) -> Result<Self> {
        let client: ollama::Client = ollama::Client::builder()
            .base_url(config.model.base_url.clone())
            .api_key(Nothing)
            .build()?;

        let agent = client
            .agent(config.model.name.clone())
            .preamble("you are helpfull personal assistant")
            .build();

        Ok(Self {
            id: id.clone(),
            agent,
        })
    }
}

impl VizierAgentImpl<openrouter::CompletionModel> {
    fn new(id: String, config: &AgentConfig) -> Result<Self> {
        let client: openrouter::Client = openrouter::Client::new(config.model.api_key.clone())?;

        let agent = client
            .agent(config.model.name.clone())
            .preamble("you are helpfull personal assistant")
            .build();

        Ok(Self {
            id: id.clone(),
            agent,
        })
    }
}
