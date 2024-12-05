use std::{ops::Deref, sync::{Arc, Mutex}};

use axum::{extract::{ws::{Message, WebSocket}, Path, Query, State, WebSocketUpgrade}, response::IntoResponse, Json};
use futures::{StreamExt,SinkExt};
use tokio::sync::broadcast;

use crate::{structs::game::{GameState,RoomState}, schema::game::GameWebsocketConnection};


pub async fn game_websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<Mutex<RoomState>>>,
    Query(params): Query<GameWebsocketConnection>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| game_websocket(socket, params.code, params.username, state.clone()))
}

async fn game_websocket(ws: WebSocket, code: String, username: String, state: Arc<Mutex<RoomState>>) {
    
    // websocket
    let (mut sink, mut stream) = ws.split();
    // player
    let (ptx, mut prx) = {
        let player_tx = broadcast::Sender::<String>::new(32);
        (player_tx.clone(), player_tx.subscribe())
    };

    // A: forward ws-stream to ptx
    let ptx_clone = ptx.clone();
    tokio::spawn(async move {
        println!("A: HERE");
        while let Some(Ok(stream_msg)) = stream.next().await {
            if let Message::Text(stream_msg) = stream_msg {
                println!("A: SEND TO PLAYER");
                if let Err(e) = ptx_clone.send(format!("{}", stream_msg)) {
                    println!("Breaking A");
                    break;
                }
            }
        }
        println!("A: OUT");
    });

    let state_clone = state.clone();
    let mut game_state = {
        GameState::get_game_state(state_clone, code.clone()).await
    };

    game_state.game_to_timer().await;
    // game_state.game_to_room().await;
    game_state.timer_to_room().await;

    // B: forward prx to gtx
    let prx_clone = ptx.subscribe();
    game_state.set_player_input(prx_clone).await;

    // D: forward rrx to sink
    game_state.room_to_sink(sink).await;

    // let mut crx = game_state.get_close_sender().await.clone().subscribe();
    // if let Ok(msg) = crx.recv().await {
    //     println!("HEHE");
    // }
    // tokio::select! {
    //     _ = crx.recv() => {}
    // }
    println!("ASTALAWISTA BABAY");


}


