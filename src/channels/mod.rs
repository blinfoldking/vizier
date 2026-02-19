use anyhow::Result;

use crate::{
    channels::{api::HTTPChannel, discord::DiscordChannel},
    config::ChannelsConfig,
    transport::VizierTransport,
};

mod api;
mod discord;

pub trait VizierChannel {
    async fn run(&mut self) -> Result<()>;
}

pub struct VizierChannels {
    config: ChannelsConfig,
    transport: VizierTransport,
}

impl VizierChannels {
    pub fn new(config: ChannelsConfig, transport: VizierTransport) -> Result<Self> {
        Ok(Self { config, transport })
    }

    pub async fn run(&self) -> Result<()> {
        if let Some(discord_config) = &self.config.discord {
            let mut discord =
                DiscordChannel::new(discord_config.clone(), self.transport.clone()).await?;
            tokio::spawn(async move {
                if let Err(e) = discord.run().await {
                    log::error!("Err{:?}", e)
                }
            });
        }

        if let Some(http) = &self.config.http {
            let mut http = HTTPChannel::new(http.clone(), self.transport.clone())?;

            tokio::spawn(async move {
                if let Err(e) = http.run().await {
                    log::error!("Err{:?}", e);
                }
            });
        }

        Ok(())
    }
}
