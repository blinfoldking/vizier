use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SessionResponse {
    pub agent_id: String,
    pub session_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatRequest {
    pub user: String,
    pub content: String,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ChatResponse {
    pub content: String,
    pub thinking: bool,
    pub timestamp: Option<DateTime<Utc>>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ChatHistory {
    request(ChatRequest),
    response(ChatResponse),
}
