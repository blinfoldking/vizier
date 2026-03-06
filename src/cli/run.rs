use std::env;

use anyhow::Result;
use clap::Args;

use crate::{
    agent::VizierAgents, channels::VizierChannels, config::VizierConfig,
    dependencies::VizierDependencies,
};

#[derive(Debug, Args, Clone)]
pub struct RunArgs {
    #[arg(
        short,
        long,
        value_name = "PATH",
        value_hint = clap::ValueHint::DirPath,
        help = "path to .vizier.toml config file",
    )]
    config: Option<std::path::PathBuf>,
}

pub async fn run_server(config: VizierConfig) -> Result<()> {
    let deps = VizierDependencies::new(config.clone()).await?;

    let mut agents = VizierAgents::new(deps.clone()).await?;
    tokio::spawn(async move {
        if let Err(err) = agents.run().await {
            log::error!("{}", err);
        }
    });

    let channels = VizierChannels::new(config.channels.clone(), deps.clone())?;
    tokio::spawn(async move {
        if let Err(err) = channels.run().await {
            log::error!("{}", err);
        }
    });

    deps.run().await?;
    Ok(())
}

pub async fn run(args: RunArgs) -> Result<()> {
    let config = VizierConfig::load(args.config.clone())?;

    if env::var("RUST_LOG").is_err() {
        pretty_env_logger::formatted_builder()
            .filter_level(log::LevelFilter::Debug)
            .filter_module("rig", log::LevelFilter::Error)
            .filter_module("serenity", log::LevelFilter::Error)
            .filter_module("sqlx", log::LevelFilter::Error)
            .filter_module("reqwest", log::LevelFilter::Error)
            .filter_module("hyper", log::LevelFilter::Error)
            .filter_module("tungstenite", log::LevelFilter::Error)
            .filter_module("sqlx", log::LevelFilter::Error)
            .filter_module("h2", log::LevelFilter::Error)
            .filter_module("tracing", log::LevelFilter::Off)
            .filter_module("rustls", log::LevelFilter::Off)
            .filter_module("surrealdb", log::LevelFilter::Off)
            .filter_module("ort", log::LevelFilter::Off)
            .filter_module("ureq", log::LevelFilter::Off)
            .init();
    } else {
        pretty_env_logger::init();
    }

    run_server(config.clone()).await?;

    Ok(())
}
