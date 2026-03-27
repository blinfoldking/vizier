use anyhow::Result;

use crate::{
    schema::{AgentId, Skill},
    storage::VizierStorage,
};

#[async_trait::async_trait]
pub trait SkillStorage {
    /// save skill as global skill when agent_id = None
    async fn save_skill(&self, agent_id: Option<AgentId>, skill: Skill) -> Result<()>;
    /// only get global skills when agent_id = None
    async fn list_skill(&self, agent_id: Option<AgentId>) -> Result<Vec<Skill>>;
    /// search skill, when agent_id = None, search from global skill only
    /// else resolve order as this: agent skill -> global skill
    async fn get_skill(&self, agent_id: Option<AgentId>, slug: String) -> Result<Option<Skill>>;
}

#[async_trait::async_trait]
impl SkillStorage for VizierStorage {
    async fn save_skill(&self, agent_id: Option<String>, skill: Skill) -> Result<()> {
        self.0.save_skill(agent_id, skill).await
    }

    async fn list_skill(&self, agent_id: Option<AgentId>) -> Result<Vec<Skill>> {
        self.0.list_skill(agent_id).await
    }

    async fn get_skill(&self, agent_id: Option<String>, slug: String) -> Result<Option<Skill>> {
        self.0.get_skill(agent_id, slug).await
    }
}
