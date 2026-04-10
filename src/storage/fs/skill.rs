use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    schema::{AgentId, Skill},
    storage::{fs::FileSystemStorage, skill::SkillStorage},
    utils::{build_glob_path, build_path},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SkillFrontMatter {
    pub name: String,
    pub author: String,
    pub description: String,
}

impl From<Skill> for SkillFrontMatter {
    fn from(value: Skill) -> Self {
        Self {
            name: value.name,
            author: value.author,
            description: value.description,
        }
    }
}

#[async_trait::async_trait]
impl SkillStorage for FileSystemStorage {
    async fn save_skill(&self, agent_id: Option<AgentId>, skill: Skill) -> Result<()> {
        let path = if let Some(agent_id) = agent_id {
            build_path(&self.workspace, &["agents", &agent_id, "skills", &skill.name, "SKILL.md"])
        } else {
            build_path(&self.workspace, &["skills", &skill.name, "SKILL.md"])
        };

        let frontmatter = SkillFrontMatter::from(skill.clone());
        crate::utils::markdown::write_markdown(
            &frontmatter,
            skill.content.clone(),
            path,
        )?;

        Ok(())
    }

    async fn list_skill(&self, agent_id: Option<AgentId>) -> Result<Vec<Skill>> {
        let mut skills = HashMap::<String, Skill>::new();

        let path = build_glob_path(&self.workspace, &["skills", "*", "SKILL.md"]);
        for entry in glob::glob(&path)? {
            let entry = entry?;

            let (frontmatter, content) =
                crate::utils::markdown::read_markdown::<SkillFrontMatter>(entry)?;

            let skill = Skill {
                name: frontmatter.name,
                author: frontmatter.author,
                description: frontmatter.description,
                agent_id: None,
                content,
            };

            skills.insert(skill.name.clone(), skill);
        }

        if let Some(agent_id) = agent_id {
            let path = build_glob_path(&self.workspace, &["agents", &agent_id, "skills", "*", "SKILL.md"]);

            for entry in glob::glob(&path)? {
                let entry = entry?;

                let (frontmatter, content) =
                    crate::utils::markdown::read_markdown::<SkillFrontMatter>(entry)?;

                let skill = Skill {
                    name: frontmatter.name,
                    author: frontmatter.author,
                    description: frontmatter.description,
                    agent_id: Some(agent_id.clone()),
                    content,
                };

                skills.insert(skill.name.clone(), skill);
            }
        }

        Ok(skills.iter().map(|(_, skill)| skill.clone()).collect())
    }
    async fn get_skill(&self, agent_id: Option<AgentId>, slug: String) -> Result<Option<Skill>> {
        if let Some(agent_id) = agent_id {
            let path = build_path(&self.workspace, &["agents", &agent_id, "skills", &slug, "SKILL.md"]);

            let (frontmatter, content) = crate::utils::markdown::read_markdown::<SkillFrontMatter>(
                path,
            )?;

            let skill = Skill {
                name: frontmatter.name,
                author: frontmatter.author,
                description: frontmatter.description,
                agent_id: Some(agent_id.clone()),
                content,
            };

            return Ok(Some(skill));
        }

        let path = build_path(&self.workspace, &["skills", &slug, "SKILL.md"]);

        let (frontmatter, content) =
            crate::utils::markdown::read_markdown::<SkillFrontMatter>(path)?;

        let skill = Skill {
            name: frontmatter.name,
            author: frontmatter.author,
            description: frontmatter.description,
            agent_id: None,
            content,
        };

        Ok(Some(skill))
    }
}
