use std::sync::Arc;

use axum::Router;

use crate::AppState;

mod health;
mod login;
mod game;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(health::get_router())
        .merge(login::get_router(app_state.clone()))
        .merge(game::get_router(app_state.db.clone()))
}