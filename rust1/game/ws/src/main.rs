use core::time;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::ws::Message;
use axum::extract::{Path, WebSocketUpgrade};
use axum::extract::{ws::WebSocket, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;
use uuid::Uuid;


#[derive(Clone, Debug)]
struct RoomState {
    room_id: String,
    occupants: u8,
    tx: broadcast::Sender<String>,
}

struct AppState {
    // channel used to send messages to all connected clients
    rooms: HashMap<String, RoomState>,
}

impl Default for AppState {
    fn default() -> Self {
        Self { rooms: HashMap::new() }
    }
}

#[tokio::main]
async fn main() {
    // Set up application state for use with with_state().
    let app = Router::new()
        .route("/ws/:room_id", get(websocket_handler))
        .with_state(Arc::new(Mutex::new(AppState::default())));

        println!("🚀 Server started successfully");
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path(room_id): Path<String>,
    State(state): State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state, room_id))
}

async fn websocket(stream: WebSocket, state: Arc<Mutex<AppState>>, room_id: String) {
    // make input broadcaster
    let game_tx = broadcast::Sender::<String>::new(32);

    let id = Uuid::new_v4().to_string();
    // let id = &id[0..4];

    // split the websocket stream into a sender (sink) and receiver (stream)
    let (sink, stream) = stream.split();

    let mut first = false;
    
    let state_clone = state.clone();
    let room_id_clone = room_id.clone();
    let room_state = {
        let mut state_guard = state_clone.lock().unwrap();
        let tx_tx = state_guard.rooms.entry(room_id.clone())
            .or_insert_with(|| {
                first=true;
                RoomState {
                    tx: broadcast::Sender::<String>::new(32),
                    occupants: 1,
                    room_id: room_id_clone
                }
            });
        if !first {
            tx_tx.occupants += 1;
        }
        tx_tx.clone()
    };
    let (tx, rx) = (room_state.tx.clone(), room_state.tx.subscribe());

    let room_state_clone = room_state.clone();
    let room_id_clone = room_id.clone();
    let game_tx_clone = game_tx.clone();
    let mut game = tokio::spawn( async move {
        game_loop(room_state_clone, game_tx_clone, room_id_clone).await;
    });

    let exit_tx = broadcast::Sender::<String>::new(32);
    let (e_tx, mut e_rx) = (exit_tx.clone(), exit_tx.subscribe());


    let mut forward_to_socket = tokio::spawn(async move {
        println!(" NEW forward to socket");
        let mut rx = rx;
        let mut sender = sink;
        loop {
            tokio::select! {
                result = rx.recv() => {
                    match result {
                        Ok(msg) => {
                            println!("forward to socket");
                            let _ = sender.send(Message::Text(msg)).await;
                        }
                        Err(_) => break, // rx is closed, exit the loop
                    }
                }
                result = e_rx.recv() => {
                    match result {
                        Ok(_msg) => {
                            println!("breaking");
                            break;
                        }
                        Err(_) => break, // e_rx is closed, exit the loop
                    }
                }
            }
        }
        println!("out of forward to socket");
    });



    let state_clone = state.clone();
    let game_tx_clone = game_tx.clone();
    let mut forward_to_client = tokio::spawn(async move {
        let mut receiver = stream;
        println!(" NEW forward to client");
        // let mut state_guard = state_clone.lock().unwrap();
        while let Some(Ok(msg)) = receiver.next().await {
            println!("forward to client");
            if let Message::Text(text) = msg {
                let _ = tx.send(format!("{}:{}", id[0..4].to_string(), text));
                // send ws-i to input-o
                let _ = game_tx_clone.send(format!("{}", text));
            }
        }
        let _ = e_tx.send(format!("close"));
        // let rs = state_guard.rooms.get(&room_id).unwrap();
        // rs.occupants -= 1;
        // if rs.occupants == 0 {
        //     state_guard.rooms.remove_entry(&room_id);
        // }
        // println!("out of forward to client");
        // forward_to_socket.abort();
        // game.abort();zz
        println!("out of forward to client");
    });




}


async fn time_loop(state: RoomState, time_tx: broadcast::Sender<String>, room_id: String) {
    // time input to room output
    let mut time_rx = time_tx.subscribe();
    println!(" NEW time loop");
    while let Ok(msg) = time_rx.recv().await {
        // let mut state_guard = state_clone.lock().unwrap();
        println!("time loop {}", msg);
        if msg == String::from("start")  {
            for i in 0..5 {
                let _= state.tx.send(format!("time {} room id {}", i, room_id.clone()));
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    }
    println!("out of time loop");
}

async fn input_loop(game_tx: broadcast::Sender<String>, time_tx: broadcast::Sender<String>) {
        println!(" NEW input loop");
        let time_tx_clone = time_tx.clone();
        // game input to time output
        let mut game_rx = game_tx.subscribe();
        while let Ok(msg) = game_rx.recv().await {
            println!("input loop");
            let _ = time_tx_clone.send(format!("{}", msg));
        }
        println!("out of input loop");
}

async fn game_loop(state: RoomState, game_tx: broadcast::Sender<String>, room_id: String) {
    println!(" NEW game loop");
    let time_tx = broadcast::Sender::<String>::new(32);

    let time_tx_clone = time_tx.clone();
    let mut input_loop = tokio::spawn(input_loop(game_tx.clone(), time_tx_clone));
    let mut time_loop = tokio::spawn(time_loop(state.clone(), time_tx.clone(), room_id.clone()));

    tokio::select! {
        _ = (&mut input_loop) => {time_loop.abort()},
        _ = (&mut time_loop) => {input_loop.abort()},
    }
}