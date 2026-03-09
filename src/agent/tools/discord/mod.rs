use std::sync::Arc;

use rig::{completion::ToolDefinition, tool::Tool};
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, Http, MessageId};

use crate::error::{VizierError, throw_vizier_error};

pub fn new_discord_tools(
    discord_token: String,
) -> (SendDiscordMessage, ReactDiscordMessage, GetDiscordMessage) {
    let http = Arc::new(Http::new(&discord_token));

    (
        SendDiscordMessage { http: http.clone() },
        ReactDiscordMessage { http: http.clone() },
        GetDiscordMessage { http: http.clone() },
    )
}

pub struct SendDiscordMessage {
    http: Arc<Http>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct SendDiscoedMessageArgs {
    #[schemars(description = "id of target discord channel")]
    channel_id: u64,

    #[schemars(description = "content of the message")]
    content: String,
}

impl Tool for SendDiscordMessage
where
    Self: Sync + Send,
{
    const NAME: &'static str = "discord_send_message";
    type Error = VizierError;
    type Args = SendDiscoedMessageArgs;
    type Output = ();

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = serde_json::to_value(schema_for!(Self::Args)).unwrap();

        ToolDefinition {
            name: Self::NAME.to_string(),
            description: format!(
                "send a discord message to a channel, avoid using this when user interact with you directly from discord"
            ),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        log::info!("discord_send_message {}", args.channel_id);

        let response = crate::utils::discord::send_message(
            self.http.clone(),
            &ChannelId::new(args.channel_id),
            args.content,
        )
        .await;

        match response {
            Ok(()) => Ok(()),
            Err(err) => throw_vizier_error("discord_send_message ", err),
        }
    }
}

pub struct ReactDiscordMessage {
    http: Arc<Http>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ReactDiscoedMessageArgs {
    #[schemars(description = "id of the target discord channel")]
    channel_id: u64,

    #[schemars(description = "id of the target discord message")]
    message_id: u64,

    #[schemars(description = "an emoji")]
    emoji: char,
}

impl Tool for ReactDiscordMessage
where
    Self: Sync + Send,
{
    const NAME: &'static str = "discord_react_message";
    type Error = VizierError;
    type Args = ReactDiscoedMessageArgs;
    type Output = ();

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = serde_json::to_value(schema_for!(Self::Args)).unwrap();

        ToolDefinition {
            name: Self::NAME.to_string(),
            description: format!("emoji react to a discord message"),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        log::info!("discord_react_message {}", args.message_id);

        let channel = ChannelId::new(args.channel_id);
        let message_id = MessageId::new(args.message_id);

        let response = channel.message(self.http.clone(), message_id).await;
        if let Err(err) = response {
            return throw_vizier_error("discord_react_message ", err);
        }

        let message = response.unwrap();
        let response = message.react(self.http.clone(), args.emoji).await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => throw_vizier_error("discord_react_message ", err),
        }
    }
}

pub struct GetDiscordMessage {
    http: Arc<Http>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct GetDiscordMessageArgs {
    #[schemars(description = "id of the target discord channel")]
    channel_id: u64,

    #[schemars(description = "id of the target discord message")]
    message_id: u64,
}

impl Tool for GetDiscordMessage
where
    Self: Sync + Send,
{
    const NAME: &'static str = "discord_get_message_by_id";
    type Error = VizierError;
    type Args = GetDiscordMessageArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = serde_json::to_value(schema_for!(Self::Args)).unwrap();

        ToolDefinition {
            name: Self::NAME.to_string(),
            description: format!("get message by message id"),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        log::info!("discord_react_message {}", args.message_id);

        let channel = ChannelId::new(args.channel_id);
        let message_id = MessageId::new(args.message_id);

        let response = channel.message(self.http.clone(), message_id).await;

        match response {
            Ok(message) => Ok(format!(
                "{}: {}",
                message.author.display_name(),
                message.content
            )),
            Err(err) => throw_vizier_error("discord_react_message ", err),
        }
    }
}
