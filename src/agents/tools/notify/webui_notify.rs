use rig::{completion::ToolDefinition, tool::Tool};
use schemars::schema_for;
use serde::{Deserialize, Serialize};

use crate::{
    error::VizierError,
    schema::{VizierChannelId, VizierResponse, VizierResponseContent, VizierSession},
    transport::VizierTransport,
};

pub struct WebUiNotifyPrimaryUser {
    agent_id: String,
    transport: VizierTransport,
}

impl WebUiNotifyPrimaryUser {
    pub fn new(agent_id: String, transport: VizierTransport) -> Self {
        Self { agent_id, transport }
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct WebUiNotifyPrimaryUserArgs {
    #[schemars(description = "content of the notification")]
    content: String,
}

impl Tool for WebUiNotifyPrimaryUser
where
    Self: Sync + Send,
{
    const NAME: &'static str = "webui_notify_primary_user";
    type Error = VizierError;
    type Args = WebUiNotifyPrimaryUserArgs;
    type Output = ();

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = serde_json::to_value(schema_for!(Self::Args)).unwrap();
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "send a notification to the primary user via WebUI".into(),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let session = VizierSession(
            self.agent_id.clone(),
            VizierChannelId::HTTP("vizier-webui".to_string()),
            Some("notification".to_string()),
        );

        let response = VizierResponse {
            timestamp: chrono::Utc::now(),
            content: VizierResponseContent::Message {
                content: args.content,
                stats: None,
            },
        };

        match self.transport.send_response(session, response).await {
            Ok(()) => Ok(()),
            Err(err) => {
                log::error!("webui_notify_primary_user: failed to send notification: {:?}", err);
                Ok(())
            }
        }
    }
}
