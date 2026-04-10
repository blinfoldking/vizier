use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode, header::AUTHORIZATION, Uri},
    middleware::Next,
    response::Response,
};

use crate::channels::http::{
    auth::{AuthService, AuthenticatedUser},
    state::HTTPState,
};
use crate::storage::user::UserStorage;

#[derive(Debug, Clone)]
pub enum AuthError {
    MissingCredentials,
    InvalidCredentials,
    InvalidToken,
    InternalError,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::MissingCredentials => write!(f, "Missing authentication credentials"),
            AuthError::InvalidCredentials => write!(f, "Invalid authentication credentials"),
            AuthError::InvalidToken => write!(f, "Invalid or expired token"),
            AuthError::InternalError => write!(f, "Internal authentication error"),
        }
    }
}

impl std::error::Error for AuthError {}

impl axum::response::IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AuthError::MissingCredentials => (StatusCode::UNAUTHORIZED, "Missing authentication credentials"),
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid authentication credentials"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
            AuthError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal authentication error"),
        };

        let body = axum::Json(serde_json::json!({
            "status": status.as_u16(),
            "message": message,
            "data": null
        }));

        (status, body).into_response()
    }
}

pub async fn require_auth(
    State(state): State<HTTPState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // Try to get credentials from Authorization header first
    let credentials_from_header = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|auth_header| {
            auth_header
                .split_once(' ')
                .map(|(auth_type, creds)| (auth_type.to_string(), creds.to_string()))
        });

    // Try to get token from query parameter if header is not present
    let credentials_from_query = if credentials_from_header.is_none() {
        extract_token_from_query(request.uri())
            .map(|token| ("bearer".to_string(), token))
    } else {
        None
    };

    // Use credentials from header or query parameter
    let (auth_type, credentials) = credentials_from_header
        .or(credentials_from_query)
        .ok_or(AuthError::MissingCredentials)?;

    let http_config = state
        .config
        .channels
        .http
        .as_ref()
        .ok_or(AuthError::InternalError)?;
    
    let auth_service = AuthService::new(http_config);

    let user = match auth_type.to_lowercase().as_str() {
        "bearer" => {
            // JWT token authentication
            let claims = auth_service
                .validate_token(&credentials)
                .map_err(|_| AuthError::InvalidToken)?;
            
            AuthenticatedUser {
                user_id: claims.sub,
                username: claims.username,
            }
        }
        "apikey" => {
            // API key authentication
            let key_hash = auth_service.hash_api_key(&credentials);
            let api_key = state
                .storage
                .get_api_key_by_hash(&key_hash)
                .await
                .map_err(|_| AuthError::InternalError)?
                .ok_or(AuthError::InvalidCredentials)?;

            // Update last used timestamp
            let _ = state
                .storage
                .update_api_key_last_used(&api_key.id)
                .await;

            // Get the user associated with this API key
            let user = state
                .storage
                .get_user(&api_key.user_id)
                .await
                .map_err(|_| AuthError::InternalError)?
                .ok_or(AuthError::InvalidCredentials)?;

            AuthenticatedUser {
                user_id: user.user_id,
                username: user.username,
            }
        }
        _ => return Err(AuthError::MissingCredentials),
    };

    // Add the authenticated user to request extensions
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}

/// Extract token from query parameter
/// Supports: ?token=<jwt_token>
fn extract_token_from_query(uri: &Uri) -> Option<String> {
    uri.query()
        .and_then(|query| {
            query
                .split('&')
                .find_map(|param| {
                    let mut parts = param.split('=');
                    match (parts.next(), parts.next()) {
                        (Some("token"), Some(token)) => Some(token.to_string()),
                        _ => None,
                    }
                })
        })
}
