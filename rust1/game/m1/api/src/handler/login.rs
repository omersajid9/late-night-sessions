use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use uuid::Uuid;

use crate::{controller::{game::Game, player::Player}, schema::login::LoginUserSchema, AppState};


pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginUserSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    // make a game row
    // make a player row
    // make a lobby row
    let mut game: Option<Game> = None;
    if let Some(code) = body.code {
        game = Some(load_game(code.clone(), state.clone()).await);
    } else {
        game = Some(new_game(state.clone()).await);
    }
    let player = load_player(body.username.clone(), game.clone().unwrap().id.unwrap().clone(), state.clone()).await;
    
    let json_response = json!({
        "player": player.clone(),
        "game": game.unwrap()
    });
    Ok(Json(json_response))
}

async fn load_player(username: String, game_id: Uuid, state: Arc<AppState>) -> Player {
    Player::new_or_load(username, game_id, state).await
}

async fn load_game(code: String, state: Arc<AppState>) -> Game {
    Game::load(code, state.clone()).await
}

async fn new_game(state: Arc<AppState>) -> Game {
    Game::new(state.clone()).await
}