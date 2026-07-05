use anyhow::Result;

use crate::schema::AgentConfig;

#[async_trait::async_trait]
pub trait AgentStorage {
    async fn list_agents(&self) -> Result<Vec<(String, AgentConfig)>>;
    async fn get_agent(&self, agent_id: &str) -> Result<Option<AgentConfig>>;
    async fn create_agent(&self, agent_id: &str, config: &AgentConfig) -> Result<()>;
    async fn update_agent(&self, agent_id: &str, config: &AgentConfig) -> Result<()>;
    async fn delete_agent(&self, agent_id: &str) -> Result<()>;

    async fn get_agent_core(&self, agent_id: &str) -> Result<Option<String>> {
        Ok(self.get_agent(agent_id).await?.and_then(|c| c.core))
    }

    async fn set_agent_core(&self, agent_id: &str, core: &str) -> Result<()> {
        let mut config = self
            .get_agent(agent_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Agent '{}' not found", agent_id))?;
        config.core = Some(core.to_string());
        self.update_agent(agent_id, &config).await
    }
}
