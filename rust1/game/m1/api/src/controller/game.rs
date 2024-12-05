use std::sync::Arc;
use serde::Serialize;
use uuid::Uuid;
use rand::prelude::*;

use crate::{model::login::GameModel, AppState};


#[derive(Debug, Serialize, Clone)]
pub struct Game {
    pub id: Option<Uuid>,
    pub code: String,
    pub active: bool,
    pub capacity: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>
}

impl Game {
    pub async fn new (data: Arc<AppState>) -> Self {
        let code = generate_game_code();
        let mut game =
        Game {
            id: None,
            code: code,
            active: false,
            capacity: 0,
            created_at: None,
            updated_at: None
        };
        let gamemodel = game.save(data).await;

        match gamemodel {
            Ok(gm) => {
                game.id = Some(gm.id);
                game.created_at = Some(gm.created_at).unwrap();
                game.updated_at = Some(gm.updated_at).unwrap();
            }
            Err(e) => {
                println!("error occurred making new game: {:?}", e)
            }
        }
        
        game
    }

    async fn save (&self, data: Arc<AppState>) -> Result<GameModel, impl std::error::Error> {
        let query_result = sqlx::query_as!(
            GameModel,
            "INSERT INTO game (code,active,capacity) VALUES ($1, $2, $3) RETURNING *",
            self.code,
            self.active,
            self.capacity
        )
        .fetch_one(&data.db)
        .await;
    
        match query_result {
            Ok(game) => Ok(game),
            Err(e) => {
                if e.to_string()
                    .contains("duplicate key value violates unique constraint")
                {
                    return Err(e);
                }
                Err(e)
            }
        }
    }

    pub async fn load (code: String, data: Arc<AppState>) -> Game {
        let query_result = sqlx::query_as!(
            GameModel,
            "SELECT * FROM game 
            WHERE code = $1",
            code.clone()
        )
        .fetch_one(&data.db)
        .await;

        if query_result.is_err() {
            return Game{id:None,code:code.clone(), active:false, capacity:0, created_at:None, updated_at:None};
        }

        let gamemodal = query_result.unwrap();
        Game {
            id: Some(gamemodal.id),
            code: code.clone(),
            active: gamemodal.active,
            capacity: gamemodal.capacity,
            created_at: gamemodal.created_at,
            updated_at: gamemodal.updated_at
        }
    }
}

fn generate_game_code() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const CODE_LENGTH: usize = 7;

    let mut rng = thread_rng();
    let code: String = (0..CODE_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CODE_LENGTH);
            CHARSET[idx] as char
        }).collect();
    code
}