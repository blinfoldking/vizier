use std::sync::Arc;

use rig::message::Message;

use crate::{
    agent::{VizierAgent, hook::VizierSessionHooks},
    config::agent::MemoryConfig,
    schema::{VizierRequest, VizierResponse},
};

#[derive(Debug, Clone)]
pub enum SessionMemory {
    Response(VizierResponse),
    Request(VizierRequest),
}

impl SessionMemory {
    fn simple(&self) -> String {
        match self {
            Self::Request(req) => format!("{}: {}", req.user, req.content),
            Self::Response(VizierResponse::Message { content, stats: _ }) => {
                format!("answer: {}", content)
            }
            _ => unimplemented!(),
        }
    }

    fn to_message(&self) -> Message {
        match self {
            Self::Request(req) => Message::user(req.to_prompt().unwrap()),
            Self::Response(VizierResponse::Message { content, stats: _ }) => {
                Message::assistant(content)
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone)]
pub struct SessionMemories {
    #[allow(unused)]
    agent_id: String,
    messages: Vec<SessionMemory>,
    session_cap: usize,
    hooks: Arc<VizierSessionHooks>,
}

impl SessionMemories {
    pub fn new(agent_id: String, config: MemoryConfig, hooks: Arc<VizierSessionHooks>) -> Self {
        Self {
            agent_id,
            messages: vec![],
            session_cap: config.max_capacity,
            hooks,
        }
    }

    fn cap_message(&mut self) {
        while self.messages.len() > self.session_cap {
            self.messages.remove(0);
        }
    }

    pub fn push_user_message(&mut self, req: VizierRequest) {
        self.messages.push(SessionMemory::Request(req));

        self.cap_message();
    }

    pub fn push_agent_message(&mut self, response: VizierResponse) {
        self.messages.push(SessionMemory::Response(response));

        self.cap_message();
    }

    pub fn recall(&self) -> Vec<SessionMemory> {
        self.messages
            .iter()
            .rev()
            .take(self.session_cap)
            .map(|item| item.clone())
            .collect()
    }

    pub fn recall_as_messages(&self) -> Vec<Message> {
        let mut res = vec![];

        res.extend(self.recall().iter().map(|item| item.to_message()));

        res
    }

    fn format_messages_for_summary(&self) -> String {
        self.messages
            .iter()
            .map(|msg| msg.simple())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn flush(&mut self) {
        self.messages.clear();
    }
}
