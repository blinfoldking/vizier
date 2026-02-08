use anyhow::Result;
use serenity::all::ChannelId;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::agent::session::VizierSession;
use crate::channels::VizierHandler;
use crate::config::DiscordChannelConfig;
use crate::transport::{VizierRequest, VizierResponse, VizierTransport};
use crate::utils::remove_think_tags;

pub struct DiscordHandler {
    transport: VizierTransport,
    config: DiscordChannelConfig,
    client: Client,
}

impl DiscordHandler {
    pub async fn new(config: DiscordChannelConfig, transport: VizierTransport) -> Result<Self> {
        let intents = GatewayIntents::all();
        let client = Client::builder(config.token.clone(), intents)
            .event_handler(Handler(transport.clone()))
            .await?;

        Ok(Self {
            config,
            client,
            transport,
        })
    }
}

impl VizierHandler for DiscordHandler {
    async fn run(&mut self) -> Result<()> {
        let transport = self.transport.clone();
        let http = self.client.http.clone();

        tokio::spawn(async move {
            loop {
                if let Ok((VizierSession::DiscordChanel(channel_id), res)) =
                    transport.response_reader.recv()
                {
                    let http = http.clone();
                    let channel_id = ChannelId::new(channel_id);

                    match res {
                        VizierResponse::StartThinking => {
                            tokio::spawn(async move {
                                let _ = channel_id.broadcast_typing(&http).await;
                            });
                        }
                        VizierResponse::Message(content) => {
                            tokio::spawn(async move {
                                if let Err(err) =
                                    channel_id.say(&http, remove_think_tags(&content)).await
                                {
                                    log::error!("{:?}", err);
                                }
                            });
                        }
                        _ => {}
                    }
                }
            }
        });

        if let Err(err) = self.client.start().await {
            log::error!("{:?}", err);
        }
        Ok(())
    }
}

struct Handler(VizierTransport);

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        if msg.author.name != "Vizier" {
            let _ = self.0.request_writer.send((
                VizierSession::DiscordChanel(msg.channel_id.get()),
                VizierRequest {
                    user: msg.author.display_name().to_string(),
                    content: msg.content,
                },
            ));
        }
    }
}
