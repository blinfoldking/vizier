use anyhow::Result;

use crate::{
    channels::discord::DiscordHandler, config::ChannelsConfig, transport::VizierTransport,
};

pub mod discord;

pub trait VizierHandler {
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
                DiscordHandler::new(discord_config.clone(), self.transport.clone()).await?;
            tokio::spawn(async move {
                if let Err(e) = discord.run().await {
                    log::error!("Err{:?}", e)
                }
            });
        }

        Ok(())
    }
}
