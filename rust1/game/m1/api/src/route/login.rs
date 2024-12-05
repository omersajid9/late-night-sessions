use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{handler::login::login, AppState};

pub fn get_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/game", get(login))
        .with_state(app_state)
}
