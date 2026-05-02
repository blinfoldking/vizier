use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Args;
use daemonize::Daemonize;
use tokio::task::JoinSet;

use crate::{
    agents::VizierAgents,
    channels::VizierChannels,
    config::{VizierConfig, provider::ProviderVariant},
    dependencies::VizierDependencies,
    scheduler::VizierScheduler,
};

#[derive(Debug, Args, Clone)]
pub struct RunArgs {
    #[arg(
        short,
        long,
        value_name = "PATH",
        value_hint = clap::ValueHint::DirPath,
        help = "path to .vizier.yaml config file",
    )]
    config: Option<std::path::PathBuf>,

    #[arg(short, long, help = "run the server in the background")]
    detached: Option<bool>,
}

#[tokio::main(flavor = "multi_thread")]
pub async fn run_server(config: VizierConfig) -> Result<()> {
    let deps = VizierDependencies::new(config.clone()).await?;
    let mut set = JoinSet::new();

    log::info!("preload all local models");
    for (_, config) in &deps.config.agents {
        if config.provider == ProviderVariant::ollama {
            let base_url = deps.config.providers.ollama.clone().unwrap().base_url;
            crate::utils::ollama::ollama_pull_model(&base_url, &config.model).await?;
        }
    }
    log::info!("preload done");

    let mut scheduler = VizierScheduler::new(deps.clone()).await?;
    set.spawn(async move {
        if let Err(err) = scheduler.run().await {
            log::error!("{}", err);
        }
    });

    let mut agents = VizierAgents::new(deps.clone()).await?;
    set.spawn(async move {
        if let Err(err) = agents.run().await {
            log::error!("{}", err);
        }
    });

    let channels = VizierChannels::new(config.channels.clone(), deps.clone())?;
    set.spawn(async move {
        if let Err(err) = channels.run().await {
            log::error!("{}", err);
        }
    });

    set.spawn(async move {
        if let Err(err) = deps.run().await {
            log::error!("{}", err);
        }
    });

    log::info!("vizier is running!");

    set.join_all().await;
    Ok(())
}

pub fn run(args: RunArgs) -> Result<()> {
    let config = VizierConfig::load(args.config.clone())?;

    let workspace = PathBuf::from(&config.workspace);

    let mut runtime_dir = workspace.clone();
    runtime_dir.push(".runtime");
    runtime_dir.push("logs");
    let _ = fs::create_dir_all(&runtime_dir);

    let mut stdout_path = runtime_dir.clone();
    stdout_path.push("out.log");
    let stdout = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(stdout_path)?;

    let mut stderr_path = runtime_dir.clone();
    stderr_path.push("err.log");
    let stderr = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(stderr_path)?;

    let config = config.clone();
    let daemonize = Daemonize::new()
        .pid_file("/tmp/vizier.pid")
        .working_directory(workspace.parent().unwrap())
        .umask(0o022)
        .stdout(stdout)
        .stderr(stderr);

    daemonize.start()?;
    let _ = run_server(config);
    Ok(())
}
