use std::sync::{Arc, Mutex};

use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};

use crate::{handler::game::game_websocket_handler, structs::game::RoomState};



pub fn get_router(db: Pool<Postgres>) -> Router {
    let mut state = RoomState::default();
    state.set_db(db);
    Router::new()
        .route("/ws/game", get(game_websocket_handler))
        .with_state(Arc::new(Mutex::new(state)))
}