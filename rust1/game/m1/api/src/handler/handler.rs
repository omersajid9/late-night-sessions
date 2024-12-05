use std::{borrow::{Borrow, BorrowMut, Cow}, sync::{Arc,Mutex}};

use axum::{
    extract::{
        ws::{CloseFrame, Message, WebSocket},
        Path, Query, State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;



use crate::{
    game::Game, model::model::NoteModel, player::Player, schema::{self, CreateNoteSchema, FilterOptions, UpdateNoteSchema}, AppState
};
use futures::{sink::SinkExt, stream::StreamExt};


use tokio::{sync::{broadcast}, time::{sleep, Duration}};

async fn update_notes_task(data: Arc<AppState>,  notes: Arc<Mutex<Vec<NoteModel>>>, mut shutdown: broadcast::Receiver<()>, ws: &mut WebSocket) {
    let mut i = 0;
    loop {
    
        let query_result = sqlx::query_as!(NoteModel, "SELECT * FROM notes ORDER by id LIMIT $1 OFFSET $2", 10 as i32, 0 as i32)
           .fetch_all(&data.db)
           .await;

        if let Ok(new_notes) = query_result {
            let mut notes_guard = notes.lock().await;
            *notes_guard = new_notes;
        }
        println!("GETTING INFO {i}");
        i += 1;

        ws
        .send(Message::Text(format!("{:?}", i)))
        .await
        .unwrap();


        tokio::select! {
            _ = tokio::time::sleep(Duration::from_nanos(1)) => {}
            _ = shutdown.recv() => break,
        }
    }
}



pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(data): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, axum::extract::State(data)))
}

async fn handle_socket(mut socket: WebSocket, State(data): State<Arc<AppState>>) {
    println!("SOCKET UPGRADED");
    let (shutdown_tx, mut shutdown_rx) = broadcast::channel::<()>(1);


    // let notes = Arc::new(Mutex::new(Vec::new()));
    // let socket = Arc::new(Mutex::new(socket));

    let data_clone = data.clone();
    

    let (mut sender, mut receiver) = socket.split();
    let mut send_task = tokio::spawn(async move {
        let n_msg = 20;
        // let mut i = 0;

        if let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(txt) = msg {
                for i in 0..5 {
                    // let mut state_write = data_clone.timer.read();
                    // In case of any websocket error, we exit.
                    if sender
                        .send(Message::Text(format!("{i}")))
                        .await
                        .is_err()
                    {
                        // return i;
                    }
                    // i += 1;
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }

            } else {

            }
        }

        // println!("Sending close to {who}...");
        // if let Err(e) = sender
        //     .send(Message::Close(Some(CloseFrame {
        //         code: axum::extract::ws::close_code::NORMAL,
        //         reason: Cow::from("Goodbye"),
        //     })))
        //     .await
        // {
        //     println!("Could not send Close due to {e}, probably it is ok?");
        // }
        // n_msg
    });

    // let mut recv_task = tokio::spawn(async move {
    //     let mut cnt = 0;
    //     while let Some(Ok(msg)) = receiver.next().await {
    //         cnt += 1;
    //         // print message and break if instructed to do so
    //         println!("USER SAID {:?}", msg)
    //         // sender
    //         //     .send(Message::Text(format!("{:?}", msg)))
    //         //     .await
    //         //     .unwrap();

    //     }
    //     cnt
    // });

    
    // tokio::spawn(update_notes_task(data.clone(), notes.clone(), shutdown_rx, socket.borrow_mut()));

    // while let Some(Ok(msg)) = socket.recv().await {
    //     match msg {
    //         Message::Text(text) => {
    //             println!("Got message {text}");
    //             let query_result = sqlx::query_as!(
    //                 NoteModel,
    //                 "INSERT INTO notes (title,content,category) VALUES ($1, $2, $3) RETURNING *",
    //                 text,
    //                 text,
    //                 text
    //             )
    //             .fetch_one(&data.db)
    //             .await;
    //             // update_notes_task(data.clone(), notes.clone()).await;
    //             let notes_guard = notes.lock().await;
    //             socket
    //                 .send(Message::Text(format!("{:?}", text)))
    //                 .await
    //                 .unwrap();
    //         }
    //         Message::Close(_) => {
    //             println!("Client disconnected");
    //             if let Err(e) = shutdown_tx.send(()) {
    //                 eprintln!("Error while sending shutdown signal: {:?}", e);
    //             }        
    //             break;
    //         }
    //         _ => {}
    //     }
    // }
}


pub async fn create_game(State(data): State<Arc<AppState>>) -> impl IntoResponse {
    let game = Game::new(data.clone()).await;
    let player = Player::new(&game, data.clone()).await;
    println!("GAME GENERATED {:?}", game);
    println!("GAME GENERATED {:?}", player);


    let json_response = serde_json::json!({
        "status": "success",
        "game metadata": game
    });
    Json(json_response)
}

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";


    let json_response = serde_json::json!({
        "status": "minahil",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn note_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(100);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        NoteModel,
        "SELECT * FROM notes ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all note items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let notes = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": notes.len(),
        "notes": notes
    });
    Ok(Json(notes))
}

pub async fn create_note_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        NoteModel,
        "INSERT INTO notes (title,content,category) VALUES ($1, $2, $3) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
        body.category.to_owned().unwrap_or("".to_string())
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success","data": json!({
                "note": note
            })});

            return Ok((StatusCode::CREATED, Json(note_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Note with that title already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

pub async fn get_note_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return Ok(Json(note));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Note with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

pub async fn edit_note_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let note = query_result.unwrap();

    let query_result = sqlx::query_as!(
        NoteModel,
        "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *",
        body.title.to_owned().unwrap_or(note.title),
        body.content.to_owned().unwrap_or(note.content),
        body.category.to_owned().unwrap_or(note.category.unwrap()),
        body.published.unwrap_or(note.published.unwrap()),
        now,
        id
    )
    .fetch_one(&data.db)
    .await
    ;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return Ok(Json(note_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

pub async fn delete_note_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM notes  WHERE id = $1", id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}
