use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use futures::future::try_join_all;
use serenity::all::{ChannelId, Http};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use text_splitter::MarkdownSplitter;

use crate::agent::session::VizierSession;
use crate::channels::VizierHandler;
use crate::config::{self, DiscordChannelConfig};
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

async fn send_message(http: Arc<Http>, channel_id: &ChannelId, content: String) -> Result<()> {
    if content.len() < 2000 {
        let channel_id = channel_id.clone();
        tokio::spawn(async move {
            let content = content.clone();
            if let Err(err) = channel_id.say(&http, content.clone()).await {
                log::error!("{:?}", err);
            }
        });

        return Ok(());
    }

    let splitter = MarkdownSplitter::new(2000);
    let content = content.clone();
    let chunks = splitter
        .chunks(&content)
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let channel_id = channel_id.clone();
    tokio::spawn(async move {
        for msg in chunks.clone() {
            if let Err(err) = channel_id.say(&http, msg).await {
                log::error!("{:?}", err);
            }
        }
    });

    Ok(())
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
                            let content = remove_think_tags(&content.clone());
                            let _ = send_message(http, &channel_id, content).await;
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
    async fn message(&self, ctx: Context, msg: Message) {
        // if let Ok(is_mention) = msg.mentions_me(ctx.http).await {
        //     if !is_mention {
        //         return;
        //     }

        let current_user = ctx.cache.current_user().discriminator;
        if msg.author.discriminator != current_user {
            let _ = self.0.request_writer.send((
                VizierSession::DiscordChanel(msg.channel_id.get()),
                VizierRequest {
                    user: msg.author.display_name().to_string(),
                    content: msg.content,
                },
            ));
        }
        // }
    }
}
