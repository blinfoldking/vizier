use rig::{
    Embed,
    completion::{CompletionModel, Prompt},
    message::Message,
};
use serde::{Deserialize, Serialize};

use crate::{
    agent::VizierAgentImpl,
    config::MemoryConfig,
    transport::{VizierRequest, VizierResponse},
};

#[derive(Debug, Clone)]
pub enum Memory {
    Response(String),
    Request(VizierRequest),
}

impl Memory {
    fn simple(&self) -> String {
        match self {
            Self::Request(req) => format!(
                r"
---
{}: {}
---
                ",
                req.user, req.content
            ),
            Self::Response(content) => format!("answer: {}", content),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Request(req) => req.to_prompt().unwrap(),
            Self::Response(content) => content.into(),
        }
    }

    fn to_message(&self) -> Message {
        match self {
            Self::Request(req) => Message::user(req.to_prompt().unwrap()),
            Self::Response(content) => Message::assistant(content),
        }
    }
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
        self.messages.push(Memory::Request(req));
    }

    pub fn push_agent(&mut self, response: String) {
        self.messages.push(Memory::Response(response));
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
        self.recall().iter().map(|item| item.to_message()).collect()
    }

    pub async fn try_summarize<T: CompletionModel>(
        &mut self,
        agent: VizierAgentImpl<T>,
    ) -> anyhow::Result<()> {
        if self.messages.len() < self.session_memory_recall_depth {
            return Ok(());
        }

        let response =        agent.agent.prompt(format!(r"
            Provided below is your recent conversation. 
            Summarize and remember it on your memory. 
            make it as concise as possible, yet maintain clarity and avoid information loss as much as possible
            {}", self.format_messages_for_summary())).await?;

        self.messages.clear();

        self.summary = Some(response.to_string());

        Ok(())
    }

    fn format_messages_for_summary(&self) -> String {
        self.messages
            .iter()
            .map(|msg| msg.simple())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn summary_prompt(&self) -> String {
        if let Some(summary) = self.summary.clone() {
            format!(
                r"
            ## Context

            Context for our current session: 

            {}
                ",
                summary,
            )
        } else {
            "".into()
        }
    }

    pub fn flush(&mut self) {
        self.messages.clear();
        self.summary = None;
    }
}
