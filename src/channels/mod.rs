use anyhow::Result;

use crate::{
    channels::{discord::DiscordChannel, http::HTTPChannel},
    config::ChannelsConfig,
    dependencies::VizierDependencies,
    transport::VizierTransport,
};

pub mod discord;
pub mod http;

pub trait VizierChannel {
    async fn run(&mut self) -> Result<()>;
}

pub struct VizierChannels {
    config: ChannelsConfig,
    deps: VizierDependencies,
}

impl VizierChannels {
    pub fn new(config: ChannelsConfig, deps: VizierDependencies) -> Result<Self> {
        Ok(Self { config, deps })
    }

    pub async fn run(&self) -> Result<()> {
        if let Some(discord_config) = &self.config.discord {
            let transport = self.deps.transport.clone();
            let discord_config = discord_config.clone();
            tokio::spawn(async move {
                let mut discord = DiscordChannel::new(discord_config.clone(), transport.clone())
                    .await
                    .unwrap();

                if let Err(e) = discord.run().await {
                    log::error!("Err{:?}", e)
                }
            });
        }

        if let Some(http) = &self.config.http {
            let mut http = HTTPChannel::new(http.clone(), self.deps.clone())?;

            tokio::spawn(async move {
                if let Err(e) = http.run().await {
                    log::error!("Err{:?}", e);
                }
            });
        }

        Ok(())
    }
}
