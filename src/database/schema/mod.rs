use chrono::Utc;
use croner::Cron;
use serde::{Deserialize, Serialize};
use surrealdb_types::{RecordId, SurrealValue};

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct Memory {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub embedding: Vec<f64>,
    pub agent_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct Message {
    pub metadata: serde_json::Value,
    pub sender: String,
    pub from_agent: bool,
    pub channel: String,
    pub content: String,
    pub timestamp: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct Task {
    pub slug: String,
    pub user: String,
    pub agent_id: String,
    pub title: String,
    pub instruction: String,
    pub is_active: bool,
    pub schedule: TaskSchedule,
    pub last_executed_at: Option<chrono::DateTime<Utc>>,
    pub timestamp: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub enum TaskSchedule {
    CronTask(String),
    OneTimeTask(chrono::DateTime<Utc>),
}
