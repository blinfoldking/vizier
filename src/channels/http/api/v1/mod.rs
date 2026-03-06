use axum::Router;
use axum::routing::get;
use reqwest::StatusCode;

use crate::channels::http::{
    api::v1::agents::agents,
    models::{self, response::api_response},
    state::HTTPState,
};

pub mod agents;

pub fn v1() -> Router<HTTPState> {
    Router::new()
        .route("/ping", get(ping))
        .nest("/agents", agents())
}

async fn ping() -> models::response::Response<String> {
    api_response(StatusCode::OK, "pong".into())
}
