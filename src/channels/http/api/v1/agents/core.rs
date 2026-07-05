use axum::{
    Extension, Router,
    extract::{Path, State},
    routing::get,
    Json,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    channels::http::{
        models::{
            self,
            response::{api_response, err_response, APIResponse},
        },
        state::HTTPState,
    },
    storage::agent::AgentStorage,
};

use super::user_can_view_agent;

pub fn core() -> Router<HTTPState> {
    Router::new()
        .route("/", get(get_core).put(update_core))
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateCoreRequest {
    content: String,
}

#[derive(Debug, Clone, Serialize, utoipa::ToSchema)]
pub struct CoreContentResponse {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, utoipa::ToSchema)]
pub struct CoreUpdateResponse {
    pub message: String,
}

#[utoipa::path(
    get,
    path = "/agents/{agent_id}/core",
    params(
        ("agent_id" = String, Path, description = "Agent ID")
    ),
    responses(
        (status = 200, description = "Get CORE content", body = APIResponse<CoreContentResponse>),
        (status = 404, description = "Agent not found", body = APIResponse<String>)
    )
)]
pub async fn get_core(
    Path(agent_id): Path<String>,
    State(state): State<HTTPState>,
    Extension(user): Extension<crate::channels::http::auth::AuthenticatedUser>,
) -> models::response::Response<CoreContentResponse> {
    let config = match state.storage.get_agent(&agent_id).await {
        Ok(Some(config)) => config,
        Ok(None) => return err_response(StatusCode::NOT_FOUND, format!("agent {agent_id} not found")),
        Err(e) => return err_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    if !user_can_view_agent(&user, &config) {
        return err_response(StatusCode::FORBIDDEN, "Access denied".into());
    }

    match state.storage.get_agent_core(&agent_id).await {
        Ok(Some(content)) => api_response(StatusCode::OK, CoreContentResponse { content }),
        Ok(None) => api_response(StatusCode::OK, CoreContentResponse { content: String::new() }),
        Err(e) => err_response(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to read CORE: {}", e)),
    }
}

#[utoipa::path(
    put,
    path = "/agents/{agent_id}/core",
    params(
        ("agent_id" = String, Path, description = "Agent ID")
    ),
    request_body = UpdateCoreRequest,
    responses(
        (status = 200, description = "CORE updated successfully", body = APIResponse<CoreUpdateResponse>),
        (status = 404, description = "Agent not found", body = APIResponse<String>),
        (status = 500, description = "Internal server error", body = APIResponse<String>)
    )
)]
pub async fn update_core(
    Path(agent_id): Path<String>,
    State(state): State<HTTPState>,
    Extension(user): Extension<crate::channels::http::auth::AuthenticatedUser>,
    Json(body): Json<UpdateCoreRequest>,
) -> models::response::Response<CoreUpdateResponse> {
    let config = match state.storage.get_agent(&agent_id).await {
        Ok(Some(config)) => config,
        Ok(None) => return err_response(StatusCode::NOT_FOUND, format!("agent {agent_id} not found")),
        Err(e) => return err_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    if !user_can_view_agent(&user, &config) {
        return err_response(StatusCode::FORBIDDEN, "Access denied".into());
    }

    match state.storage.set_agent_core(&agent_id, &body.content).await {
        Ok(_) => api_response(StatusCode::OK, CoreUpdateResponse { message: "CORE updated successfully".to_string() }),
        Err(e) => err_response(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to update CORE: {}", e)),
    }
}
