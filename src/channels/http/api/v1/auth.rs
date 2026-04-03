use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::channels::http::{
    auth::{AuthService, AuthenticatedUser},
    models::{self, response::api_response},
    state::HTTPState,
};
use crate::storage::user::UserStorage;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub expires_in_days: Option<i64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateApiKeyResponse {
    pub id: String,
    pub name: String,
    pub key: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ApiKeyResponse {
    pub id: String,
    pub name: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

pub async fn login(
    State(state): State<HTTPState>,
    Json(req): Json<LoginRequest>,
) -> models::response::Response<LoginResponse> {
    let http_config = match &state.config.channels.http {
        Some(cfg) => cfg,
        None => {
            return models::response::err_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "HTTP channel not configured".to_string(),
            );
        }
    };

    let auth_service = AuthService::new(http_config);

    // Get user from storage
    let user = match state.storage.get_user(&req.username).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return models::response::err_response(
                StatusCode::UNAUTHORIZED,
                "Invalid username or password".to_string(),
            );
        }
        Err(e) => {
            log::error!("Failed to get user: {}", e);
            return models::response::err_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            );
        }
    };

    // Verify password
    match auth_service.verify_password(&req.password, &user.password_hash) {
        Ok(true) => {}
        Ok(false) => {
            return models::response::err_response(
                StatusCode::UNAUTHORIZED,
                "Invalid username or password".to_string(),
            );
        }
        Err(e) => {
            log::error!("Failed to verify password: {}", e);
            return models::response::err_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            );
        }
    }

    // Generate JWT token
    match auth_service.generate_token(&user.id, &user.username) {
        Ok(token) => api_response(StatusCode::OK, LoginResponse { token }),
        Err(e) => {
            log::error!("Failed to generate token: {}", e);
            models::response::err_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to generate token".to_string(),
            )
        }
    }
}

pub async fn change_password(
    State(state): State<HTTPState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(req): Json<ChangePasswordRequest>,
) -> models::response::Response<String> {
    let http_config = match &state.config.channels.http {
        Some(cfg) => cfg,
        None => {
            return models::response::err_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "HTTP channel not configured".to_string(),
            );
        }
    };

    let auth_service = AuthService::new(http_config);

    // Get current user data to verify current password
    let current_user = match state.storage.get_user(&user.username).await {
        Ok(Some(u)) => u,
        _ => {
            return models::response::err_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get user data".to_string(),
            );
        }
    };

    // Verify current password
    match auth_service.verify_password(&req.current_password, &current_user.password_hash) {
        Ok(true) => {}
        Ok(false) => {
            return models::response::err_response(
                StatusCode::UNAUTHORIZED,
                "Current password is incorrect".to_string(),
            );
        }
        Err(e) => {
            log::error!("Failed to verify password: {}", e);
            return models::response::err_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            );
        }
    }

    // Hash new password
    let new_password_hash = match auth_service.hash_password(&req.new_password) {
        Ok(hash) => hash,
        Err(e) => {
            log::error!("Failed to hash new password: {}", e);
            return models::response::err_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to process new password".to_string(),
            );
        }
    };

    // Update password in storage
    match state.storage.update_password(&user.user_id, &new_password_hash).await {
        Ok(_) => api_response(StatusCode::NO_CONTENT, String::new()),
        Err(e) => {
            log::error!("Failed to update password: {}", e);
            models::response::err_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update password".to_string(),
            )
        }
    }
}

pub async fn create_api_key(
    State(state): State<HTTPState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(req): Json<CreateApiKeyRequest>,
) -> models::response::Response<CreateApiKeyResponse> {
    let http_config = match &state.config.channels.http {
        Some(cfg) => cfg,
        None => {
            return models::response::err_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "HTTP channel not configured".to_string(),
            );
        }
    };

    let auth_service = AuthService::new(http_config);

    // Generate new API key (raw key shown once, hash stored)
    let (raw_key, key_hash) = auth_service.generate_api_key();

    // Calculate expiration
    let expires_at = req.expires_in_days.map(|days| Utc::now() + Duration::days(days));

    // Store API key
    let api_key = match state
        .storage
        .create_api_key(&user.user_id, &req.name, &key_hash, expires_at)
        .await
    {
        Ok(key) => key,
        Err(e) => {
            log::error!("Failed to create API key: {}", e);
            return models::response::err_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create API key".to_string(),
            );
        }
    };

    api_response(
        StatusCode::CREATED,
        CreateApiKeyResponse {
            id: api_key.id,
            name: api_key.name,
            key: raw_key, // Only time this is shown
            expires_at: api_key.expires_at,
            created_at: api_key.created_at,
        },
    )
}

pub async fn list_api_keys(
    State(state): State<HTTPState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> models::response::Response<Vec<ApiKeyResponse>> {
    let keys = match state.storage.list_api_keys(&user.user_id).await {
        Ok(keys) => keys,
        Err(e) => {
            log::error!("Failed to list API keys: {}", e);
            return models::response::err_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to list API keys".to_string(),
            );
        }
    };

    let response: Vec<ApiKeyResponse> = keys
        .into_iter()
        .map(|k| ApiKeyResponse {
            id: k.id,
            name: k.name,
            expires_at: k.expires_at,
            created_at: k.created_at,
            last_used_at: k.last_used_at,
        })
        .collect();

    api_response(StatusCode::OK, response)
}

pub async fn delete_api_key(
    State(state): State<HTTPState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(key_id): Path<String>,
) -> models::response::Response<String> {
    // First verify the key belongs to the user
    let keys = match state.storage.list_api_keys(&user.user_id).await {
        Ok(keys) => keys,
        Err(e) => {
            log::error!("Failed to list API keys: {}", e);
            return models::response::err_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to verify API key ownership".to_string(),
            );
        }
    };

    if !keys.iter().any(|k| k.id == key_id) {
        return models::response::err_response(
            StatusCode::NOT_FOUND,
            "API key not found".to_string(),
        );
    }

    match state.storage.delete_api_key(&key_id).await {
        Ok(_) => api_response(StatusCode::NO_CONTENT, String::new()),
        Err(e) => {
            log::error!("Failed to delete API key: {}", e);
            models::response::err_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete API key".to_string(),
            )
        }
    }
}
