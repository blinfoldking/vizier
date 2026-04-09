use teloxide::Bot;
use teloxide::prelude::*;

use crate::error::VizierError;

const MAX_MESSAGE_LENGTH: usize = 4096;

fn escape_markdown_v2(text: &str) -> String {
    let reserved = [
        '_', '[', ']', '(', ')', '~', '`', '+', '-', '=', '|', '{', '}', '.', '!',
    ];
    let mut escaped = String::with_capacity(text.len() * 2);
    for c in text.chars() {
        if reserved.contains(&c) {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}

pub async fn send_message(bot: &Bot, chat_id: &ChatId, content: String) -> Result<(), VizierError> {
    let escaped_content = escape_markdown_v2(&content);

    if escaped_content.len() < MAX_MESSAGE_LENGTH {
        if let Err(err) = bot
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .send_message(*chat_id, escaped_content)
            .await
        {
            log::error!("{:?}", err);
        }
        return Ok(());
    }

    let chunks: Vec<String> = escaped_content
        .chars()
        .collect::<Vec<char>>()
        .chunks(MAX_MESSAGE_LENGTH)
        .map(|chunk| chunk.iter().collect())
        .collect();

    for msg in chunks {
        if let Err(err) = bot
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .send_message(*chat_id, msg)
            .await
        {
            log::error!("{:?}", err);
        }
    }

    Ok(())
}

