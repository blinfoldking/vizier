extern crate pretty_env_logger;
#[allow(unused)]
#[macro_use]
extern crate log;

use std::process;

use anyhow::Result;

mod agent;
mod channels;
mod config;
mod constant;
mod database;
mod dependencies;
mod error;
mod transport;
mod utils;
mod vizier;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    vizier::cli().await?;
    println!("vizier exited!");
    process::exit(0);
}
