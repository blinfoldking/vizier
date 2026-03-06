use anyhow::Result;

use clap::{Parser, Subcommand};

mod onboard;
mod run;

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
    Run(run::RunArgs),
    /// Onboard new user, and generate configurations
    Onboard(onboard::OnboardArgs),
    /// generate new config, non-interactively
    Configure,
    /// Open tui client
    Tui,
}

pub async fn start() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Onboard(args) => onboard::onboard(args.clone()).await?,
        Commands::Run(args) => run::run(args.clone()).await?,
        _ => {
            unimplemented!("TODO: unimplemented");
        }
    }

    Ok(())
}
