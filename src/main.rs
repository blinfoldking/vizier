extern crate pretty_env_logger;
#[allow(unused)]
#[macro_use]
extern crate log;

use anyhow::Result;

mod agent;
mod channels;
mod config;
mod constant;
mod error;
mod transport;
mod utils;
mod vizier;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    vizier::cli().await
}
