use std::collections::HashMap;

use anyhow::Result;

use crate::{
    schema::{AgentId, Skill},
    storage::{skill::SkillStorage, surreal::SurrealStorage},
};

#[async_trait::async_trait]
impl SkillStorage for SurrealStorage {
    async fn save_skill(&self, agent_id: Option<AgentId>, skill: Skill) -> Result<()> {
        let mut slug = skill.name.clone();
        if let Some(agent_id) = agent_id {
            slug = format!("{}/", agent_id);
        }

        let _: Option<Skill> = self.conn.upsert(("skill", slug)).content(skill).await?;

        Ok(())
    }

    async fn list_skill(&self, agent_id: Option<AgentId>) -> Result<Vec<Skill>> {
        let mut skills = HashMap::<String, Skill>::new();
        // get global skills
        let mut res = self
            .conn
            .query("SELECT * FROM skill WHERE agent_id == NULL")
            .await?;

        let list: Vec<Skill> = res.take(0)?;
        list.iter().for_each(|skill| {
            skills.insert(skill.name.clone(), skill.clone());
        });

        // get agent skills
        if let Some(agent_id) = agent_id {
            let mut res = self
                .conn
                .query("SELECT * FROM skill WHERE agent_id = $agent_id")
                .bind(("agent_id", agent_id))
                .await?;

            let list: Vec<Skill> = res.take(0)?;
            list.iter().for_each(|skill| {
                skills.insert(skill.name.clone(), skill.clone());
            });
        }

        Ok(skills.iter().map(|(_, skill)| skill.clone()).collect())
    }

    async fn get_skill(&self, agent_id: Option<AgentId>, slug: String) -> Result<Option<Skill>> {
        // get agent skills
        if let Some(agent_id) = agent_id {
            let mut res = self
                .conn
                .query("SELECT * FROM skill WHERE agent_id = $agent_id AND slug = $slug")
                .bind(("agent_id", agent_id))
                .bind(("slug", slug))
                .await?;

            let skill: Option<Skill> = res.take(0)?;
            return Ok(skill);
        }

        // search global skills
        let mut res = self
            .conn
            .query("SELECT * FROM skill WHERE agent_id = NULL AND slug = $slug")
            .bind(("slug", slug))
            .await?;

        let skill: Option<Skill> = res.take(0)?;
        Ok(skill)
    }
}
