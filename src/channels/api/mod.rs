use anyhow::Result;
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    agent::session::VizierSession,
    channels::{
        VizierChannel,
        api::response::{Response, api_response, err_response},
    },
    config::APIChannelConfig,
    transport::{VizierRequest, VizierResponse, VizierTransport, VizierTransportChannel},
};

mod response;

pub struct APIChannel {
    config: APIChannelConfig,
    transport: VizierTransport,
}

impl APIChannel {
    pub fn new(config: APIChannelConfig, transport: VizierTransport) -> Result<Self> {
        Ok(Self { config, transport })
    }
}

impl VizierChannel for APIChannel {
    async fn run(&mut self) -> Result<()> {
        let app = Router::new()
            .route("/", get(ping))
            .route("/chat", post(chat))
            .with_state(self.transport.clone());

        let listener =
            tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.config.port)).await?;

        axum::serve(listener, app).await?;

        Ok(())
    }
}

async fn ping() -> response::Response<String> {
    api_response(StatusCode::OK, "pong".into())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ChatRequest {
    session_id: String,
    user: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ChatResponse {
    session_id: String,
    content: String,
}

// TODO: currently only works on single session
async fn chat(
    transport: State<VizierTransport>,
    request: Json<ChatRequest>,
) -> Response<ChatResponse> {
    if let Err(err) = transport
        .0
        .send_request(
            VizierSession::API(request.0.session_id.clone()),
            VizierRequest {
                user: request.user.clone(),
                content: request.content.clone(),
            },
        )
        .await
    {
        return err_response(StatusCode::INTERNAL_SERVER_ERROR, err.to_string());
    }

    loop {
        match transport.0.read_response(VizierTransportChannel::API).await {
            Err(err) => return err_response(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            Ok((_, VizierResponse::Message(content))) => {
                return api_response(
                    StatusCode::OK,
                    ChatResponse {
                        session_id: request.session_id.clone(),
                        content,
                    },
                );
            }
            _ => {}
        }
    }
}
