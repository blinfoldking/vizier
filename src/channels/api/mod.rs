use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use axum::{
    Router,
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    routing::{any, get, post},
};
use chrono::Utc;
use flume::{Receiver, Sender};
use futures::{SinkExt, StreamExt};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;

use crate::{
    agent::session::VizierSession,
    channels::{VizierChannel, api::response::api_response},
    config::HTTPChannelConfig,
    transport::{VizierRequest, VizierResponse, VizierTransport, VizierTransportChannel},
};

pub mod response;

pub struct HTTPChannel {
    config: HTTPChannelConfig,
    transport: VizierTransport,
}

impl HTTPChannel {
    pub fn new(config: HTTPChannelConfig, transport: VizierTransport) -> Result<Self> {
        Ok(Self { config, transport })
    }
}

type ChatRequestTransport = (
    Sender<(String, ChatRequest)>,
    Receiver<(String, ChatRequest)>,
);
type ChatReponseTransport = (Sender<ChatResponse>, Receiver<ChatResponse>);

#[derive(Debug, Clone)]
struct ChatTransport {
    requests: ChatRequestTransport,
    reponses: Arc<Mutex<HashMap<String, ChatReponseTransport>>>,
}

impl ChatTransport {
    pub fn new() -> Self {
        Self {
            requests: flume::unbounded(),
            reponses: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl VizierChannel for HTTPChannel {
    async fn run(&mut self) -> Result<()> {
        let chat_transport = ChatTransport::new();

        let app = Router::new()
            .route("/", get(ping))
            .route("/session", post(create_session))
            .route("/session/{session_id}", post(create_custom_session))
            .route("/session/{session_id}/chat", any(ws))
            .with_state(chat_transport.clone());

        let listener =
            tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.config.port)).await?;

        let req_transport = self.transport.clone();
        let req_chat_transport = chat_transport.clone();
        let request_handle = tokio::spawn(async move {
            while let Ok((session_id, request)) = req_chat_transport.requests.1.recv_async().await {
                let metadata = json!({
                    "sent_at": Utc::now().to_string(),
                    "websocket_session_id": session_id,
                });

                let _ = req_transport
                    .send_request(
                        VizierSession::HTTP(session_id),
                        VizierRequest {
                            user: request.user,
                            content: request.content,
                            metadata,
                            ..Default::default()
                        },
                    )
                    .await;
            }
        });

        let res_transport = self.transport.clone();
        let res_chat_transport = chat_transport.clone();
        let response_handle = tokio::spawn(async move {
            while let Ok((VizierSession::HTTP(session_id), response)) = res_transport
                .read_response(VizierTransportChannel::HTTP)
                .await
            {
                let response_transport = res_chat_transport.reponses.lock().await;
                if let Some(transport) = response_transport.get(&session_id).clone() {
                    let (writer, _) = transport.clone();
                    match response {
                        VizierResponse::Thinking => {
                            let _ = writer
                                .send_async(ChatResponse {
                                    thinking: true,
                                    ..Default::default()
                                })
                                .await;
                        }
                        VizierResponse::Message(content) => {
                            let _ = writer
                                .send_async(ChatResponse {
                                    content,
                                    thinking: false,
                                    ..Default::default()
                                })
                                .await;
                        }
                    }
                }
            }
        });

        let server = axum::serve(listener, app);
        log::info!("http listening on port {}", self.config.port);
        server.await?;
        request_handle.abort();
        response_handle.abort();

        Ok(())
    }
}

async fn ping() -> response::Response<String> {
    api_response(StatusCode::OK, "pong".into())
}

async fn create_custom_session(
    Path(session_id): Path<String>,
    sessions: State<ChatTransport>,
) -> response::Response<SessionResponse> {
    sessions
        .0
        .reponses
        .lock()
        .await
        .insert(session_id.clone(), flume::unbounded());

    api_response(StatusCode::OK, SessionResponse { session_id })
}

async fn create_session(sessions: State<ChatTransport>) -> response::Response<SessionResponse> {
    let session_id = uuid::Uuid::new_v4().to_string();
    sessions
        .0
        .reponses
        .lock()
        .await
        .insert(session_id.clone(), flume::unbounded());

    api_response(StatusCode::OK, SessionResponse { session_id })
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SessionResponse {
    pub session_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatRequest {
    pub user: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ChatResponse {
    pub content: String,
    pub thinking: bool,
}

async fn ws(
    Path(session_id): Path<String>,
    ws: WebSocketUpgrade,
    state: State<ChatTransport>,
) -> axum::response::Response {
    log::debug!("connect {}", session_id);
    let responses = state.reponses.lock().await;
    let session = responses.get(&session_id);
    if let Some(session) = session {
        let requests = state.requests.clone();
        let responses = session.clone();
        return ws.on_upgrade(|socket| handle_socket(socket, session_id, requests, responses));
    } else {
        axum::response::Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("not found".into())
            .unwrap()
    }
}

async fn handle_socket(
    socket: WebSocket,
    session_id: String,
    requests: ChatRequestTransport,
    responses: ChatReponseTransport,
) {
    let (mut writer, mut reader) = socket.split();
    let handle = tokio::spawn(async move {
        loop {
            if let Ok(response) = responses.1.recv_async().await {
                let _ = writer
                    .send(axum::extract::ws::Message::Text(
                        serde_json::to_string(&response).unwrap().into(),
                    ))
                    .await;
            }
        }
    });

    loop {
        if let Some(Ok(message)) = reader.next().await {
            match message {
                Message::Text(text) => {
                    if let Ok(request) = serde_json::from_str::<ChatRequest>(&text.to_string()) {
                        log::debug!("{:?}", request);
                        let _ = requests.0.send_async((session_id.clone(), request)).await;
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    }

    handle.abort();
}
