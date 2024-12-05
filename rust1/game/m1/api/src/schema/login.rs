use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUserSchema {
    pub username: String,
    pub code: Option<String>
}