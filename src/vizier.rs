use std::{fs, path::PathBuf, str::FromStr};

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use crate::{
    agent::VizierAgents,
    channels::VizierChannels,
    config::VizierConfig,
    constant::{AGENT_MD, BOOT_MD, CONTEXT_MD, IDENT_MD, USER_MD},
    transport::VizierTransport,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run vizier agents, servers and channels
    Run(RunArgs),
    /// Onboard new user, and generate configurations
    Onboard,
    /// generate new config, non-interactively
    Configure,
    /// Open tui client
    Tui,
}

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

    #[arg(long, help = "serve with tui")]
    tui: bool,
}

pub async fn cli() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Onboard => (),
        Commands::Run(args) => run(args.clone()).await?,
        _ => {
            unimplemented!("TODO: unimplemented");
        }
    }

    Ok(())
}

async fn run(args: RunArgs) -> Result<()> {
    let config = VizierConfig::load(args.config)?;
    let transport = VizierTransport::new();

    init_workspace(config.workspace.clone());

    let mut agents = VizierAgents::new(
        config.workspace.clone(),
        config.agents.clone(),
        config.memory.clone(),
        config.tools.clone(),
        transport.clone(),
    )?;
    tokio::spawn(async move {
        if let Err(err) = agents.run().await {
            log::error!("{}", err);
        }
    });

    let channels = VizierChannels::new(config.channels.clone(), transport.clone())?;
    tokio::spawn(async move {
        if let Err(err) = channels.run().await {
            log::error!("{}", err);
        }
    });

    transport.run().await?;

    loop {}
}

pub fn init_workspace(path: String) {
    let boot_path = PathBuf::from(format!("{}/BOOT.md", path.clone()));
    let user_path = PathBuf::from(format!("{}/USER.md", path.clone()));
    let agent_path = PathBuf::from(format!("{}/AGENT.md", path.clone()));
    let ident_path = PathBuf::from(format!("{}/IDENT.md", path.clone()));
    let context_path = PathBuf::from(format!("{}/CONTEXT.md", path.clone()));

    let create_file_if_not_exists = |path: PathBuf, content: &str| {
        if !path.exists() {
            let _ = fs::write(path, content);
        }
    };

    let path = PathBuf::from(&path);

    if !path.exists() {
        let _ = std::fs::create_dir_all(path);
    }

    create_file_if_not_exists(boot_path, BOOT_MD);
    create_file_if_not_exists(user_path, USER_MD);
    create_file_if_not_exists(agent_path, AGENT_MD);
    create_file_if_not_exists(ident_path, IDENT_MD);
    create_file_if_not_exists(context_path, CONTEXT_MD);
}
