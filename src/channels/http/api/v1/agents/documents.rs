use axum::{
    Router,
    extract::{Path, State},
    routing::{get, put},
    Json,
};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    channels::http::{
        models::{
            self,
            response::{api_response, err_response},
        },
        state::HTTPState,
    },
};

pub fn documents() -> Router<HTTPState> {
    Router::new()
        .route("/agent", get(get_agent_doc).put(update_agent_doc))
        .route("/identity", get(get_identity_doc).put(update_identity_doc))
        .route("/heartbeat", get(get_heartbeat_doc).put(update_heartbeat_doc))
}

#[derive(Debug, Deserialize)]
pub struct UpdateDocumentRequest {
    content: String,
}

fn get_document_path(workspace: &str, agent_id: &str, doc_name: &str) -> String {
    format!("{}/agents/{}/{}", workspace, agent_id, doc_name)
}

fn read_document(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

fn write_document(path: &str, content: &str) -> Result<(), std::io::Error> {
    std::fs::write(path, content)
}

pub async fn get_agent_doc(
    Path(agent_id): Path<String>,
    State(state): State<HTTPState>,
) -> models::response::Response<serde_json::Value> {
    if !state.config.is_agent_exists(&agent_id) {
        return err_response(StatusCode::NOT_FOUND, format!("agent {agent_id} not found"));
    }

    let path = get_document_path(&state.config.workspace, &agent_id, "AGENT.md");
    match read_document(&path) {
        Ok(content) => api_response(StatusCode::OK, serde_json::json!({ "content": content })),
        Err(e) => err_response(StatusCode::NOT_FOUND, format!("failed to read AGENT.md: {}", e)),
    }
}

pub async fn update_agent_doc(
    Path(agent_id): Path<String>,
    State(state): State<HTTPState>,
    Json(body): Json<UpdateDocumentRequest>,
) -> models::response::Response<serde_json::Value> {
    if !state.config.is_agent_exists(&agent_id) {
        return err_response(StatusCode::NOT_FOUND, format!("agent {agent_id} not found"));
    }

    let path = get_document_path(&state.config.workspace, &agent_id, "AGENT.md");
    match write_document(&path, &body.content) {
        Ok(_) => api_response(
            StatusCode::OK,
            serde_json::json!({ "message": "AGENT.md updated successfully" }),
        ),
        Err(e) => err_response(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to update AGENT.md: {}", e)),
    }
}

pub async fn get_identity_doc(
    Path(agent_id): Path<String>,
    State(state): State<HTTPState>,
) -> models::response::Response<serde_json::Value> {
    if !state.config.is_agent_exists(&agent_id) {
        return err_response(StatusCode::NOT_FOUND, format!("agent {agent_id} not found"));
    }

    let path = get_document_path(&state.config.workspace, &agent_id, "IDENTITY.md");
    match read_document(&path) {
        Ok(content) => api_response(StatusCode::OK, serde_json::json!({ "content": content })),
        Err(e) => err_response(StatusCode::NOT_FOUND, format!("failed to read IDENTITY.md: {}", e)),
    }
}

pub async fn update_identity_doc(
    Path(agent_id): Path<String>,
    State(state): State<HTTPState>,
    Json(body): Json<UpdateDocumentRequest>,
) -> models::response::Response<serde_json::Value> {
    if !state.config.is_agent_exists(&agent_id) {
        return err_response(StatusCode::NOT_FOUND, format!("agent {agent_id} not found"));
    }

    let path = get_document_path(&state.config.workspace, &agent_id, "IDENTITY.md");
    match write_document(&path, &body.content) {
        Ok(_) => api_response(
            StatusCode::OK,
            serde_json::json!({ "message": "IDENTITY.md updated successfully" }),
        ),
        Err(e) => err_response(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to update IDENTITY.md: {}", e)),
    }
}

pub async fn get_heartbeat_doc(
    Path(agent_id): Path<String>,
    State(state): State<HTTPState>,
) -> models::response::Response<serde_json::Value> {
    if !state.config.is_agent_exists(&agent_id) {
        return err_response(StatusCode::NOT_FOUND, format!("agent {agent_id} not found"));
    }

    let path = get_document_path(&state.config.workspace, &agent_id, "HEARTBEAT.md");
    match read_document(&path) {
        Ok(content) => api_response(StatusCode::OK, serde_json::json!({ "content": content })),
        Err(e) => err_response(StatusCode::NOT_FOUND, format!("failed to read HEARTBEAT.md: {}", e)),
    }
}

pub async fn update_heartbeat_doc(
    Path(agent_id): Path<String>,
    State(state): State<HTTPState>,
    Json(body): Json<UpdateDocumentRequest>,
) -> models::response::Response<serde_json::Value> {
    if !state.config.is_agent_exists(&agent_id) {
        return err_response(StatusCode::NOT_FOUND, format!("agent {agent_id} not found"));
    }

    let path = get_document_path(&state.config.workspace, &agent_id, "HEARTBEAT.md");
    match write_document(&path, &body.content) {
        Ok(_) => api_response(
            StatusCode::OK,
            serde_json::json!({ "message": "HEARTBEAT.md updated successfully" }),
        ),
        Err(e) => err_response(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to update HEARTBEAT.md: {}", e)),
    }
}