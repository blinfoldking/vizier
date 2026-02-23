use axum::extract::{
    Path, State, WebSocketUpgrade,
    ws::{Message, WebSocket},
};
use futures::{SinkExt, StreamExt};
use reqwest::StatusCode;

use crate::channels::http::{
    models::{
        self,
        response::api_response,
        session::{ChatRequest, SessionResponse},
    },
    state::{ChatReponseTransport, ChatRequestTransport, ChatTransport},
};

pub async fn create_custom_session(
    Path(session_id): Path<String>,
    sessions: State<ChatTransport>,
) -> models::response::Response<SessionResponse> {
    sessions
        .0
        .reponses
        .lock()
        .await
        .insert(session_id.clone(), flume::unbounded());

    api_response(StatusCode::OK, SessionResponse { session_id })
}

pub async fn create_session(
    sessions: State<ChatTransport>,
) -> models::response::Response<SessionResponse> {
    let session_id = uuid::Uuid::new_v4().to_string();
    sessions
        .0
        .reponses
        .lock()
        .await
        .insert(session_id.clone(), flume::unbounded());

    api_response(StatusCode::OK, SessionResponse { session_id })
}

pub async fn chat(
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

pub async fn handle_socket(
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
