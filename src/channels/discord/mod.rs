use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use serde_json::json;
use serenity::all::{
    ChannelId, Command, CreateCommand, CreateInteractionResponseMessage, Http, Interaction, Ready,
};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use text_splitter::MarkdownSplitter;

use crate::agent::session::VizierSession;
use crate::channels::VizierChannel;
use crate::config::DiscordChannelConfig;
use crate::transport::{VizierRequest, VizierResponse, VizierTransport, VizierTransportChannel};
use crate::utils::remove_think_tags;

pub struct DiscordChannel {
    transport: VizierTransport,
    config: DiscordChannelConfig,
    client: Client,
}

impl DiscordChannel {
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
        let content = content.clone();
        if let Err(err) = channel_id.say(&http, content.clone()).await {
            log::error!("{:?}", err);
        }

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
    })
    .await?;

    Ok(())
}

impl VizierChannel for DiscordChannel {
    async fn run(&mut self) -> Result<()> {
        let transport = self.transport.clone();
        let token = self.config.token.clone();

        tokio::spawn(async move {
            loop {
                let http = Arc::new(Http::new(&token));
                if let Ok((VizierSession::DiscordChanel(channel_id), res)) = transport
                    .read_response(VizierTransportChannel::Discord)
                    .await
                {
                    let http = http;
                    let channel_id = ChannelId::new(channel_id);

                    match res {
                        VizierResponse::Thinking => {
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
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let ping = CreateCommand::new("ping").description("a simple ping");
        let lobotomy = CreateCommand::new("lobotomy")
            .description("Reset current conversation in this channel");
        let help = CreateCommand::new("help").description("How to use me");

        let _ = Command::create_global_command(ctx.http.clone(), ping).await;
        let _ = Command::create_global_command(ctx.http.clone(), lobotomy).await;
        let _ = Command::create_global_command(ctx.http.clone(), help).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            if command.data.name == "ping" {
                let _ = command
                    .create_response(
                        ctx.http.clone(),
                        serenity::all::CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new().content("Pong!"),
                        ),
                    )
                    .await;
            }

            if command.data.name == "lobotomy" {
                let _ = command
                    .create_response(
                        ctx.http.clone(),
                        serenity::all::CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new().content("NOOOOOOOOO!!!"),
                        ),
                    )
                    .await;

                let metadata = json!({
                    "sent_at": Utc::now().to_string(),
                    "discord_channel_id": command.channel_id.to_string(),
                });

                if let Err(err) = self
                    .0
                    .send_request(
                        VizierSession::DiscordChanel(command.channel_id.get()),
                        VizierRequest {
                            user: format!(
                                "@{} (DiscordId: {})",
                                command.user.display_name(),
                                command.user.id.to_string()
                            ),
                            content: "/lobotomy".into(),
                            is_silent_read: true,
                            metadata,
                            ..Default::default()
                        },
                    )
                    .await
                {
                    log::error!("{}", err)
                }
            }

            if command.data.name == "help" {
                if let Err(err) = command
                    .create_response(
                        ctx.http.clone(),
                        serenity::all::CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new().content(
                                r#"
Just mention `@vizier` when you need to summon me.
I will only read the chat otherwise.
If I am halucinating, feel free to `/lobotomy` me
                            "#,
                            ),
                        ),
                    )
                    .await
                {
                    log::error!("{}", err)
                }
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if let Ok(is_mention) = msg.mentions_me(ctx.http).await {
            let transport = self.0.clone();
            let current_user = ctx.cache.current_user().discriminator;
            if msg.author.discriminator == current_user {
                return;
            }

            let metadata = json!({
                "sent_at": Utc::now().to_string(),
                "message_id": msg.id.to_string(),
                "discord_channel_id": msg.channel_id.to_string(),
            });

            if !is_mention {
                tokio::spawn(async move {
                    if let Err(err) = transport
                        .send_request(
                            VizierSession::DiscordChanel(msg.channel_id.get()),
                            VizierRequest {
                                user: format!(
                                    "@{} (DiscordId: {})",
                                    msg.author.display_name(),
                                    msg.author.id.to_string()
                                ),
                                content: msg.content,
                                is_silent_read: true,
                                metadata,
                                ..Default::default()
                            },
                        )
                        .await
                    {
                        log::error!("{}", err)
                    }
                });

                return;
            }

            tokio::spawn(async move {
                if let Err(err) = transport
                    .send_request(
                        VizierSession::DiscordChanel(msg.channel_id.get()),
                        VizierRequest {
                            user: format!(
                                "@{} (DiscordId: {})",
                                msg.author.display_name(),
                                msg.author.id.to_string()
                            ),
                            content: msg.content,
                            metadata,

                            ..Default::default()
                        },
                    )
                    .await
                {
                    log::error!("{}", err)
                }
            });
        }
    }
}
