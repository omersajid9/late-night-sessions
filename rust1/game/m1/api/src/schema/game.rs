use serde::Deserialize;




#[derive(Deserialize)]
pub struct GameWebsocketConnection {
    pub username: String,
    pub code: String
}