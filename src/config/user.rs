use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserConfig {
    pub username: String,
    #[serde(default)]
    pub discord_id: String,
    #[serde(default)]
    pub discord_username: String,
    #[serde(default)]
    pub telegram_username: String,
    pub alias: Vec<String>,
}
