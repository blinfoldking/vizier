use axum::Router;

use crate::channels::http::state::HTTPState;

pub mod v1;

pub fn api() -> Router<HTTPState> {
    Router::new().nest("/v1", v1::v1())
}
