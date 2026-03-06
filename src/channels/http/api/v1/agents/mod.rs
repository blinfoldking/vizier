use axum::{Router, extract::State, routing::get};
use reqwest::StatusCode;

use crate::{
    channels::http::{
        models::{self, response::api_response},
        state::HTTPState,
    },
    config::VizierConfig,
};

mod memory;
mod session;

use memory::memory;
use session::session;

impl VizierConfig {
    fn is_agent_exists(&self, agent_id: &String) -> bool {
        self.agents.get(agent_id).is_some()
    }
}

pub fn agents() -> Router<HTTPState> {
    Router::new()
        .route("/", get(list_agents))
        .nest("/{agent_id}/memory", memory())
        .nest("/{agent_id}/session", session())
}

async fn list_agents(State(state): State<HTTPState>) -> models::response::Response<Vec<String>> {
    let res = state
        .config
        .agents
        .iter()
        .map(|(key, _)| key.clone())
        .collect();

    api_response(StatusCode::OK, res)
}
