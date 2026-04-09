use teloxide::prelude::*;
use teloxide::Bot;
use teloxide::sugar::request::RequestReplyExt;
use rig::{completion::ToolDefinition, tool::Tool};
use schemars::schema_for;
use serde::{Deserialize, Serialize};

use crate::error::{throw_vizier_error, VizierError};

pub fn new_telegram_tools(bot_token: String) -> (SendTelegramMessage, ReactTelegramMessage, GetTelegramMessage) {
    let bot = Bot::new(bot_token);

    (
        SendTelegramMessage { bot: bot.clone() },
        ReactTelegramMessage { bot: bot.clone() },
        GetTelegramMessage { bot: bot.clone() },
    )
}

pub struct SendTelegramMessage {
    bot: Bot,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct SendTelegramMessageArgs {
    #[schemars(description = "id of target telegram chat")]
    chat_id: i64,

    #[schemars(description = "content of the message")]
    content: String,
}

impl Tool for SendTelegramMessage
where
    Self: Sync + Send,
{
    const NAME: &'static str = "telegram_send_message";
    type Error = VizierError;
    type Args = SendTelegramMessageArgs;
    type Output = ();

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = serde_json::to_value(schema_for!(Self::Args)).unwrap();

        ToolDefinition {
            name: Self::NAME.to_string(),
            description: format!(
                "send a telegram message to a chat, avoid using this when user interact with you directly from telegram"
            ),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let response = crate::utils::telegram::send_message(
            &self.bot,
            &ChatId(args.chat_id),
            args.content,
        )
        .await;

        match response {
            Ok(()) => Ok(()),
            Err(err) => throw_vizier_error("telegram_send_message ", err),
        }
    }
}

pub struct ReactTelegramMessage {
    bot: Bot,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ReactTelegramMessageArgs {
    #[schemars(description = "id of the target telegram chat")]
    chat_id: i64,

    #[schemars(description = "id of the target telegram message")]
    message_id: i64,

    #[schemars(description = "an emoji reaction")]
    emoji: String,
}

impl Tool for ReactTelegramMessage
where
    Self: Sync + Send,
{
    const NAME: &'static str = "telegram_react_message";
    type Error = VizierError;
    type Args = ReactTelegramMessageArgs;
    type Output = ();

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = serde_json::to_value(schema_for!(Self::Args)).unwrap();

        ToolDefinition {
            name: Self::NAME.to_string(),
            description: format!("emoji react to a telegram message"),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let chat_id = ChatId(args.chat_id);
        let message_id = teloxide::types::MessageId(args.message_id as i32);

        let response = self
            .bot
            .send_message(chat_id, format!("Reaction: {}", args.emoji))
            .reply_to(message_id)
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => throw_vizier_error("telegram_react_message ", err),
        }
    }
}

pub struct GetTelegramMessage {
    bot: Bot,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct GetTelegramMessageArgs {
    #[schemars(description = "id of the target telegram chat")]
    chat_id: i64,

    #[schemars(description = "id of the target telegram message")]
    message_id: i64,
}

impl Tool for GetTelegramMessage
where
    Self: Sync + Send,
{
    const NAME: &'static str = "telegram_get_message_by_id";
    type Error = VizierError;
    type Args = GetTelegramMessageArgs;
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
        let chat_id = ChatId(args.chat_id);
        let message_id = teloxide::types::MessageId(args.message_id as i32);

        let response = self
            .bot
            .edit_message_text(chat_id, message_id, "Retrieving message...")
            .await;

        match response {
            Ok(msg) => Ok(format!(
                "{}: {}",
                msg.from().map(|u| u.full_name()).unwrap_or_else(|| "Unknown".into()),
                msg.text().unwrap_or("")
            )),
            Err(err) => throw_vizier_error("telegram_get_message ", err),
        }
    }
}