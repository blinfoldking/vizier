use anyhow::Result;
use rig::{
    client::Nothing,
    providers::{anthropic, deepseek, gemini, ollama, openai, openrouter},
};

use crate::{
    agents::agent::model::{VizierModelBuilder, VizierModelImpl},
    dependencies::VizierDependencies,
};

#[async_trait::async_trait]
impl VizierModelBuilder<ollama::Client> for VizierModelImpl<ollama::Client> {
    async fn init_client(_agent_id: String, deps: VizierDependencies) -> Result<ollama::Client> {
        let base_url = deps.config.providers.ollama.clone().unwrap().base_url;

        let client: ollama::Client = ollama::Client::builder()
            .base_url(base_url)
            .api_key(Nothing)
            .build()?;

        Ok(client)
    }
}

#[async_trait::async_trait]
impl VizierModelBuilder<openrouter::Client> for VizierModelImpl<openrouter::Client> {
    async fn init_client(
        _agent_id: String,
        deps: VizierDependencies,
    ) -> Result<openrouter::Client> {
        let client: openrouter::Client =
            openrouter::Client::new(deps.config.providers.openrouter.clone().unwrap().api_key)?;

        Ok(client)
    }
}

#[async_trait::async_trait]
impl VizierModelBuilder<deepseek::Client> for VizierModelImpl<deepseek::Client> {
    async fn init_client(_agent_id: String, deps: VizierDependencies) -> Result<deepseek::Client> {
        let client: deepseek::Client =
            deepseek::Client::new(deps.config.providers.deepseek.clone().unwrap().api_key)?;

        Ok(client)
    }
}

#[async_trait::async_trait]
impl VizierModelBuilder<anthropic::Client> for VizierModelImpl<anthropic::Client> {
    async fn init_client(_agent_id: String, deps: VizierDependencies) -> Result<anthropic::Client> {
        let client: anthropic::Client =
            anthropic::Client::new(deps.config.providers.anthropic.clone().unwrap().api_key)?;

        Ok(client)
    }
}

#[async_trait::async_trait]
impl VizierModelBuilder<openai::Client> for VizierModelImpl<openai::Client> {
    async fn init_client(_agent_id: String, deps: VizierDependencies) -> Result<openai::Client> {
        let client: openai::Client =
            if let Some(base_url) = deps.config.providers.openai.clone().unwrap().base_url {
                let api_key = deps.config.providers.openai.clone().unwrap().api_key;
                openai::Client::builder()
                    .base_url(base_url)
                    .api_key(api_key)
                    .build()?
            } else {
                openai::Client::new(deps.config.providers.openai.clone().unwrap().api_key)?
            };

        Ok(client)
    }
}

#[async_trait::async_trait]
impl VizierModelBuilder<gemini::Client> for VizierModelImpl<gemini::Client> {
    async fn init_client(_agent_id: String, deps: VizierDependencies) -> Result<gemini::Client> {
        let client: gemini::Client =
            gemini::Client::new(deps.config.providers.gemini.clone().unwrap().api_key)?;

        Ok(client)
    }
}
