mod health;

use axum::{routing::get, Router};

use crate::state::ApiState;

pub fn router(state: ApiState) -> Router {
    Router::new().route("/health", get(health::health_check))
}
