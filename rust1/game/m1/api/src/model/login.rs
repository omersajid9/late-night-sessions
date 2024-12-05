use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;




#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(non_snake_case)]
pub struct GameModel {
    pub id: Uuid,
    pub code: String,
    pub active: bool,
    pub capacity: i32,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(non_snake_case)]
pub struct PlayerModel {
    pub id: Uuid,
    pub username: String,
    pub game_id: Uuid,
    pub logged_in: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}