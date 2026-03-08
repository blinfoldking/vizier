pub type AgentId = String;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VizierSession(pub AgentId, pub SessionId);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SessionId {
    DiscordChanel(u64),
    HTTP(String),
    Task(String),
}
