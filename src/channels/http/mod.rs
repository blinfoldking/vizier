use anyhow::Result;
use axum::{
    Router,
    routing::{any, get, post},
};

use crate::{
    channels::{VizierChannel, http::state::ChatTransport},
    config::HTTPChannelConfig,
    transport::VizierTransport,
};

pub mod models;

mod api;
mod state;
mod webui;

pub struct HTTPChannel {
    config: HTTPChannelConfig,
    transport: VizierTransport,
}

impl HTTPChannel {
    pub fn new(config: HTTPChannelConfig, transport: VizierTransport) -> Result<Self> {
        Ok(Self { config, transport })
    }
}

impl VizierChannel for HTTPChannel {
    async fn run(&mut self) -> Result<()> {
        let chat_transport = ChatTransport::new();

        let app = Router::new()
            // webui
            .route("/", get(webui::index))
            .route("/{*path}", get(webui::assets))
            // api
            .route("/api/v1/ping", get(api::v1::ping))
            // session api
            .route("/api/v1/session", post(api::v1::session::create_session))
            .route(
                "/api/v1/session/{session_id}",
                post(api::v1::session::create_custom_session),
            )
            .route(
                "/api/v1/session/{session_id}/chat",
                any(api::v1::session::chat),
            )
            .with_state(chat_transport.clone());

        let listener =
            tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.config.port)).await?;

        let transport = self.transport.clone();
        let transport_handle = tokio::spawn(async move {
            let _ = chat_transport.run(transport).await;
        });

        let server = axum::serve(listener, app);
        log::info!("http listening on port {}", self.config.port);

        server.await?;
        transport_handle.abort();

        Ok(())
    }
}
