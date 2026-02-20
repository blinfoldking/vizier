extern crate pretty_env_logger;
#[allow(unused)]
#[macro_use]
extern crate log;

use std::env;

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
            .init();
    } else {
        pretty_env_logger::init();
    }

    vizier::cli().await
}
