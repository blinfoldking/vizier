use anyhow::Result;
use log::info;
use rig::{
    client::{CompletionClient, Nothing},
    providers::{deepseek, ollama, openrouter},
};

use crate::{
    agent::{
        agent_impl::{
            VizierAgentImpl,
            system_prompt::{boot::boot_md, init_workspace},
        },
        tools::VizierTools,
    },
    dependencies::VizierDependencies,
    utils::agent_workspace,
};

async fn ollama_pull_model(base_url: &str, model: &str) -> Result<()> {
    let pull_url = format!("{}/api/pull", base_url);
    let http_client = reqwest::Client::new();

    info!("Pulling Ollama model '{}'...", model);

    let resp = http_client
        .post(&pull_url)
        .json(&serde_json::json!({
            "name": model,
            "stream": false
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!(
            "Failed to pull Ollama model '{}': {}",
            model,
            resp.text().await.unwrap_or_default()
        );
    }

    info!("Ollama model '{}' is ready", model);
    Ok(())
}

impl VizierAgentImpl<ollama::CompletionModel> {
    pub async fn new(id: String, deps: VizierDependencies) -> Result<Self> {
        let agent_workspace = agent_workspace(&deps.config.workspace, &id);
        init_workspace(agent_workspace.clone());

        let agent_config = deps.config.agents.get(&id).unwrap();

        let base_url = deps.config.providers.ollama.clone().unwrap().base_url;

        ollama_pull_model(&base_url, &agent_config.model).await?;

        let client: ollama::Client = ollama::Client::builder()
            .base_url(base_url)
            .api_key(Nothing)
            .build()?;

        let boot = boot_md(agent_config);

        let tool = VizierTools::new(id.clone(), deps.clone())?;

        let agent = client
            .agent(agent_config.model.clone())
            .name(&agent_config.name.clone())
            .preamble(&boot)
            .tool_server_handle(tool.handle)
            .default_max_turns(agent_config.turn_depth)
            .build();

        Ok(Self {
            id: id.clone(),
            agent,
            workspace: deps.config.workspace.clone(),
            primary_user: deps.config.primary_user.clone(),
            silent_read_initiative_chance: agent_config.silent_read_initiative_chance,
        })
    }
}

impl VizierAgentImpl<openrouter::CompletionModel> {
    pub async fn new(id: String, deps: VizierDependencies) -> Result<Self> {
        let agent_workspace = agent_workspace(&deps.config.workspace, &id);
        init_workspace(agent_workspace.clone());

        let agent_config = deps.config.agents.get(&id).unwrap();

        let client: openrouter::Client =
            openrouter::Client::new(deps.config.providers.openrouter.clone().unwrap().api_key)?;

        let boot = boot_md(agent_config);

        let tool = VizierTools::new(id.clone(), deps.clone())?;

        let agent = client
            .agent(agent_config.model.clone())
            .name(&agent_config.name)
            .preamble(&boot)
            .tool_server_handle(tool.handle)
            .default_max_turns(agent_config.turn_depth)
            .build();

        Ok(Self {
            id: id.clone(),
            agent,
            workspace: deps.config.workspace.clone(),
            primary_user: deps.config.primary_user.clone(),
            silent_read_initiative_chance: agent_config.silent_read_initiative_chance,
        })
    }
}

impl VizierAgentImpl<deepseek::CompletionModel> {
    pub async fn new(id: String, deps: VizierDependencies) -> Result<Self> {
        let agent_workspace = agent_workspace(&deps.config.workspace, &id);
        init_workspace(agent_workspace.clone());

        let agent_config = deps.config.agents.get(&id).unwrap();

        let client: deepseek::Client =
            deepseek::Client::new(deps.config.providers.deepseek.clone().unwrap().api_key)?;

        let boot = boot_md(agent_config);

        let tool = VizierTools::new(id.clone(), deps.clone())?;

        let agent = client
            .agent(agent_config.model.clone())
            .name(&agent_config.name)
            .preamble(&boot)
            .tool_server_handle(tool.handle)
            .default_max_turns(agent_config.turn_depth)
            .build();

        Ok(Self {
            id: id.clone(),
            agent,
            workspace: deps.config.workspace.clone(),
            primary_user: deps.config.primary_user.clone(),
            silent_read_initiative_chance: agent_config.silent_read_initiative_chance,
        })
    }
}
