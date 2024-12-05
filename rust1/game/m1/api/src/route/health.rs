use axum::{routing::get, Router};

use crate::handler::health::health_checker_handler;

pub fn get_router() -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
}