use mongodb::{Client, options::ClientOptions};
use rig::{
    Embed, OneOrMany,
    completion::{CompletionModel, Prompt},
    message::{Message, UserContent},
};
use serde::{Deserialize, Serialize};

use crate::{agent::VizierAgentImpl, config::MemoryConfig, transport::VizierRequest};

#[derive(Debug, Clone)]
pub struct Memory {
    sender: String,
    content: String,
    is_agent: bool,
}

#[derive(Debug, Clone)]
pub struct SessionMemory {
    messages: Vec<Memory>,
    session_memory_recall_depth: usize,
    summary: Option<String>,
}

impl SessionMemory {
    pub fn new(config: MemoryConfig) -> Self {
        Self {
            messages: vec![],
            session_memory_recall_depth: config.session_memory_recall_depth,
            summary: None,
        }
    }

    pub fn push_user_message(&mut self, req: VizierRequest) {
        self.messages.push(Memory {
            sender: req.user,
            content: req.content,
            is_agent: false,
        });
    }

    pub fn push_agent(&mut self, agent: String, response: String) {
        self.messages.push(Memory {
            sender: agent,
            content: response,
            is_agent: true,
        });
    }

    pub fn recall(&self) -> Vec<Memory> {
        self.messages
            .iter()
            .rev()
            .take(self.session_memory_recall_depth)
            .map(|item| item.clone())
            .collect()
    }

    pub fn recall_as_messages(&self) -> Vec<Message> {
        self.recall()
            .iter()
            .map(|item| {
                if item.is_agent {
                    Message::assistant_with_id(item.sender.clone(), item.content.clone())
                } else {
                    Message::user(format!("{}: {}", item.sender, item.content))
                }
            })
            .collect()
    }

    pub async fn try_summarize<T: CompletionModel>(
        &mut self,
        agent: VizierAgentImpl<T>,
    ) -> anyhow::Result<()> {
        if self.messages.len() < self.session_memory_recall_depth {
            return Ok(());
        }

        let response =        agent.agent.prompt(format!(r"
            Provided below is our recent conversation. 
            Summarize it, make it as concise as possible, yet maintain clarity and avoid information loss as much as possible
            {}", self.format_messages_for_summary())).await?;

        self.messages.clear();

        self.summary = Some(response.to_string());

        Ok(())
    }

    fn format_messages_for_summary(&self) -> String {
        self.messages
            .iter()
            .map(|msg| format!("- {}: {:?}\n", msg.sender, msg.content))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn summary_prompt(&self) -> String {
        match self.summary.clone() {
            Some(summary) => format!(
                r"
            ## Context

            Context for our current session: {}
                ",
                summary
            ),
            _ => "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Embed, Clone)]
pub struct VizierMemory {
    #[embed]
    content: String,
    timestamp: String,
}

pub struct VizierMemoryStore {}
impl VizierMemoryStore {
    // TODO: add additional vector store
    pub async fn new(config: MemoryConfig) -> anyhow::Result<()> {
        let client_options = ClientOptions::parse("mongodb://localhost").await?;
        let client = Client::with_options(client_options)?;
        let db = client.database("vizier");

        Ok(())
    }
}
