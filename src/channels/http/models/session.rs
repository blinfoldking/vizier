use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::schema::VizierResponse;

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

impl From<VizierResponse> for ChatResponse {
    fn from(value: VizierResponse) -> Self {
        match value {
            VizierResponse::Thinking => Self {
                content: "".into(),
                thinking: true,
                timestamp: Some(Utc::now()),
            },
            VizierResponse::Message {
                content: content,
                stats: _,
            } => Self {
                content: content,
                thinking: false,
                timestamp: Some(Utc::now()),
            },
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ChatHistory {
    request(ChatRequest),
    response(ChatResponse),
}
