use anyhow::Result;
use clap::Parser;

use crate::{
    agent::VizierAgents,
    channels::VizierChannels,
    config::VizierConfig,
    transport::{VizierTransport, VizierTransportConstructor},
};

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(
        short,
        long,
        value_name = "PATH",
        value_hint = clap::ValueHint::DirPath,
        help = "path to .vizier.toml config file",
    )]
    config: Option<std::path::PathBuf>,
}

pub async fn run() -> Result<()> {
    let args = Args::parse();

    let config = VizierConfig::load(args.config)?;
    let transport = VizierTransport::new();

    let mut agents = VizierAgents::new(config.agents.clone(), transport.clone())?;
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
