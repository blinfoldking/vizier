use axum::{
    Router,
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
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
    storage::memory::MemoryStorage,
};

pub fn memory() -> Router<HTTPState> {
    Router::new()
        .route("/", get(get_all_memories))
        .route("/", post(create_memory))
        .route("/query", get(query_memories))
        .route("/{slug}", get(get_memory_detail))
        .route("/{slug}", put(update_memory))
        .route("/{slug}", delete(delete_memory))
}

#[derive(Debug, Deserialize)]
pub struct CreateMemoryRequest {
    title: String,
    content: String,
    slug: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemoryRequest {
    title: String,
    content: String,
}

#[derive(Debug, Deserialize)]
pub struct QueryMemoryRequest {
    query: String,
    #[serde(default = "default_limit")]
    limit: usize,
    #[serde(default = "default_threshold")]
    threshold: f64,
}

fn default_limit() -> usize {
    10
}

fn default_threshold() -> f64 {
    0.5
}

pub async fn get_all_memories(
    Path(agent_id): Path<String>,
    State(state): State<HTTPState>,
) -> models::response::Response<Vec<serde_json::Value>> {
    if !state.config.is_agent_exists(&agent_id) {
        return err_response(StatusCode::NOT_FOUND, format!("agent {agent_id} not found"));
    }

    match state.storage.get_all_agent_memory(agent_id).await {
        Ok(memory) => {
            let response = memory
                .iter()
                .map(|memory| {
                    serde_json::json!({
                        "agent_id": memory.agent_id,
                        "slug": memory.slug,
                        "title": memory.title,
                        "timestamp": memory.timestamp
                    })
                })
                .collect();

            api_response(StatusCode::OK, response)
        }
        _ => err_response(StatusCode::NOT_FOUND, "Not Found".into()),
    }
}

pub async fn create_memory(
    Path(agent_id): Path<String>,
    State(state): State<HTTPState>,
    Json(body): Json<CreateMemoryRequest>,
) -> models::response::Response<serde_json::Value> {
    if !state.config.is_agent_exists(&agent_id) {
        return err_response(StatusCode::NOT_FOUND, format!("agent {agent_id} not found"));
    }

    match state
        .storage
        .write_memory(agent_id.clone(), body.slug, body.title.clone(), body.content)
        .await
    {
        Ok(_) => api_response(
            StatusCode::CREATED,
            serde_json::json!({
                "agent_id": agent_id,
                "title": body.title,
                "message": "memory created successfully"
            }),
        ),
        Err(e) => err_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

pub async fn update_memory(
    Path((agent_id, slug)): Path<(String, String)>,
    State(state): State<HTTPState>,
    Json(body): Json<UpdateMemoryRequest>,
) -> models::response::Response<serde_json::Value> {
    if !state.config.is_agent_exists(&agent_id) {
        return err_response(StatusCode::NOT_FOUND, format!("agent {agent_id} not found"));
    }

    // Check if memory exists
    match state.storage.get_memory_detail(agent_id.clone(), slug.clone()).await {
        Ok(None) => {
            return err_response(StatusCode::NOT_FOUND, format!("memory {slug} not found"));
        }
        Err(e) => return err_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        _ => {}
    }

    // Update by writing new content with the same slug
    match state
        .storage
        .write_memory(agent_id.clone(), Some(slug.clone()), body.title, body.content)
        .await
    {
        Ok(_) => api_response(
            StatusCode::OK,
            serde_json::json!({
                "agent_id": agent_id,
                "slug": slug,
                "message": "memory updated successfully"
            }),
        ),
        Err(e) => err_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

pub async fn query_memories(
    Path(agent_id): Path<String>,
    Query(params): Query<QueryMemoryRequest>,
    State(state): State<HTTPState>,
) -> models::response::Response<Vec<serde_json::Value>> {
    if !state.config.is_agent_exists(&agent_id) {
        return err_response(StatusCode::NOT_FOUND, format!("agent {agent_id} not found"));
    }

    match state
        .storage
        .query_memory(agent_id, params.query, params.limit, params.threshold)
        .await
    {
        Ok(memories) => {
            let response = memories
                .iter()
                .map(|memory| {
                    serde_json::json!({
                        "agent_id": memory.agent_id,
                        "slug": memory.slug,
                        "title": memory.title,
                        "content": memory.content,
                        "timestamp": memory.timestamp
                    })
                })
                .collect();

            api_response(StatusCode::OK, response)
        }
        Err(e) => err_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

pub async fn get_memory_detail(
    Path((agent_id, slug)): Path<(String, String)>,
    State(state): State<HTTPState>,
) -> models::response::Response<serde_json::Value> {
    if !state.config.is_agent_exists(&agent_id) {
        return err_response(StatusCode::NOT_FOUND, format!("agent {agent_id} not found"));
    }

    match state.storage.get_memory_detail(agent_id, slug).await {
        Ok(Some(memory)) => api_response(
            StatusCode::OK,
            serde_json::json!({
                "agent_id": memory.agent_id,
                "slug": memory.slug,
                "title": memory.title,
                "content": memory.content,
                "timestamp": memory.timestamp
            }),
        ),
        _ => err_response(StatusCode::NOT_FOUND, "Not Found".into()),
    }
}

pub async fn delete_memory(
    Path((agent_id, slug)): Path<(String, String)>,
    State(state): State<HTTPState>,
) -> models::response::Response<String> {
    if !state.config.is_agent_exists(&agent_id) {
        return err_response(StatusCode::NOT_FOUND, format!("agent {agent_id} not found"));
    }

    match state.storage.delete_memory(agent_id, slug.clone()).await {
        Ok(_) => api_response(StatusCode::OK, format!("{slug} deleted")),
        _ => err_response(StatusCode::NOT_FOUND, "Not Found".into()),
    }
}
