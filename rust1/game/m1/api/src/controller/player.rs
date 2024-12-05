use std::sync::Arc;

use serde::Serialize;
use uuid::Uuid;

use crate::{model::login::PlayerModel, AppState};
use chrono::Utc;

#[derive(Debug, Serialize, Clone)]
pub struct Player {
    pub id: Option<Uuid>,
    pub username: String,
    pub game_id: Uuid,
    pub logged_in: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Player {
    pub async fn new_or_load(username: String, game_id: Uuid, data: Arc<AppState>) -> Self {

        let query_result = sqlx::query_as!(
            PlayerModel,
            "SELECT * 
            FROM player 
            WHERE game_id = $1",
            game_id.clone()
        )
        .fetch_all(&data.db)
        .await;

        let mut player = Player {
            id: None,
            game_id: game_id.clone(),
            logged_in: false,
            username: username.clone(),
            created_at: None,
            updated_at: None
        };

        match query_result {
            Ok(playermodels) => {
                if playermodels.is_empty() {
                    player.logged_in = true;
                    let _playermodel = player.new(data.clone()).await.unwrap();
                }
                for playermodel in playermodels {
                    if playermodel.username == username.clone() {
                        if !playermodel.logged_in {
                            player.logged_in = true;
                            let _playermodel = player.load(data.clone()).await.unwrap();
                        }
                    } else {
                        player.logged_in = true;
                        let _playermodel = player.new(data.clone()).await.unwrap();
                    }
                }
            }
            Err(err) => {
                println!("Error getting player in new_or_load {}", err.to_string());
            },
        }
        player
    }

    pub async fn new(&mut self, data: Arc<AppState>) -> Result<(), impl std::error::Error> {

        let query_result = sqlx::query_as!(
            PlayerModel,
            "INSERT INTO player 
            (game_id,logged_in,username) 
            VALUES ($1, $2, $3) 
            RETURNING *",
            self.game_id,
            self.logged_in,
            self.username
        )
        .fetch_one(&data.db)
        .await;

        match query_result {
            Ok(game) => {
                self.id = Some(game.id);
                self.created_at = game.created_at;
                self.updated_at = game.updated_at;
                Ok(())
            }
            Err(e) => {
                if e.to_string()
                    .contains("duplicate key value violates unique constraint")
                {
                    return Err(e);
                }
                return Err(e);
            }
        }
    }

    async fn load(&mut self, data: Arc<AppState>) -> Result<(), impl std::error::Error> {
        let query_result = sqlx::query_as!(
            PlayerModel,
            "UPDATE player 
            SET logged_in = $1,
            updated_at = $2
            WHERE game_id = $3
            AND username = $4
            RETURNING *",
            self.logged_in,
            Utc::now(),
            self.game_id,
            self.username
        )
        .fetch_one(&data.db)
        .await;

        match query_result {
            Ok(game) => {
                self.id = Some(game.id);
                self.created_at = game.created_at;
                self.updated_at = game.updated_at;
                Ok(())
            }
            Err(e) => {
                if e.to_string()
                    .contains("duplicate key value violates unique constraint")
                {
                    return Err(e);
                }
                return Err(e);
            }
        }
    }
}