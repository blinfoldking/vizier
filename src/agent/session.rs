#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum VizierSession {
    DiscordChanel(u64),
    API(String),
}
