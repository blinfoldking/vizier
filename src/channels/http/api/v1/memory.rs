use axum::extract::State;
use reqwest::StatusCode;

use crate::{
    channels::http::{
        models::{
            self,
            response::{api_response, err_response},
        },
        state::HTTPState,
    },
    database::{VizierDatabases, schema::Memory},
};

pub async fn get_all_memories(
    State(state): State<HTTPState>,
) -> models::response::Response<Vec<Memory>> {
    match state.db.conn.select("memory").await {
        Ok(data) => api_response(StatusCode::OK, data),
        _ => err_response(StatusCode::NOT_FOUND, "Not Found".into()),
    }
}
