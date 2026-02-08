use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use crate::{
    agent::VizierAgents, channels::VizierChannels, config::VizierConfig, transport::VizierTransport,
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
    Run(ServeArgs),
    /// Onboard new user, and generate configurations
    Onboard,
    /// generate new config, non-interactively
    Configure,
    /// Open tui client
    Tui,
}

#[derive(Debug, Args)]
pub struct ServeArgs {
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

pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Onboard => (),
        Commands::Run(args) => {
            let config = VizierConfig::load(args.config.clone())?;
            let transport = VizierTransport::new();

            let mut agents = VizierAgents::new(
                config.agents.clone(),
                config.memory.clone(),
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

            // TODO: log transport
            loop {}
        }
        _ => {
            unimplemented!("TODO: unimplemented");
        }
    }

    Ok(())
}
