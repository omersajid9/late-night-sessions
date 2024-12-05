use axum::{http::StatusCode, response::IntoResponse, Json};


pub async fn health_checker_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";


    let json_response = serde_json::json!({
        "status": "hhehe",
        "message": MESSAGE
    });

    Ok(Json(json_response))
}
