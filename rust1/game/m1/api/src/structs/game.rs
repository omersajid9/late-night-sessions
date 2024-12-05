use std::{collections::HashMap, sync::{Arc,Mutex}};

use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, FutureExt, SinkExt, TryFutureExt};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tokio::sync::broadcast;

#[derive(Debug)]
pub struct RoomState {
    state: HashMap<String, GameState>,
    db: Option<Pool<Postgres>>
}

impl Default for RoomState {
    fn default() -> Self {
        Self { state: HashMap::new(), db: None }
    }
}

impl RoomState {
    pub fn set_db(&mut self, db: Pool<Postgres>) {
        self.db = Some(db);
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    room: broadcast::Sender<String>,
    game: broadcast::Sender<String>,
    timer: broadcast::Sender<String>,
    close: broadcast::Sender<String>,
    capacity: usize,
    innings_number: usize,
    match_state: Option<MatchState>,
}

impl Default for GameState {
    fn default() -> Self {
        Self { 
            room: broadcast::Sender::<String>::new(32),
            game: broadcast::Sender::<String>::new(32),
            timer: broadcast::Sender::<String>::new(32),
            close: broadcast::Sender::<String>::new(32),
            capacity: 0,
            innings_number: 0,
            match_state: None
         }
    }
}

impl GameState {

    pub async fn get_room_sender(&self) -> broadcast::Sender<String> {
        self.room.clone()
    }
    pub async fn get_game_sender(&self) -> broadcast::Sender<String> {
        self.game.clone()
    }
    pub async fn get_timer_sender(&self) -> broadcast::Sender<String> {
        self.timer.clone()
    }
    pub async fn get_close_sender(&self) -> broadcast::Sender<String> {
        self.close.clone()
    }
}

impl GameState {
    // B: forward prx to gtx
    pub async fn set_player_input(&mut self, mut prx: broadcast::Receiver<String>) {
        let gtx = self.game.clone();
        let ctx = self.close.clone();
        let capacity = self.capacity.clone();
        println!("B: HERE");
        let a = tokio::spawn(async move {
            while let Ok(prx_msg) = prx.recv().await {
                println!("B: Player sent to Game");
                if let Err(e) = gtx.send(format!("{}", prx_msg)) {
                    println!("B: Breaking set_player_input");
                    break;
                };
            }
            // if capacity == 1 {
            // let _ = ctx.send("close".to_string());
            println!("B: EXITING EVERYTHING");
            // }
            println!("B: OUT");
        });
        self.capacity -= 1;
    }


    pub async fn game_logic(&mut self) {
        
        // open thread
        // receive from grx
    }

    // C: grx to ttx
    pub async fn game_to_timer(&mut self) {
        let mut grx = self.game.subscribe();
        let ttx = self.timer.clone();
        let mut crx = self.close.subscribe();
        if self.capacity > 1 {
            return;
        }
        println!("C: HERE");
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Ok(grx_msg) = grx.recv() => {
                        if grx_msg == String::from("start") {
                            println!("C: Game sent to Time");
                            if let Err(e) = ttx.send(format!("{}", grx_msg)) {
                                println!("C: Breaking game_to_timer");
                                break;
                            }
                        }
                    },
                    Ok(_) = crx.recv() => {
                        break;
                    }
                }
            }
            println!("C: OUT");
        });
    }

    // D: forward rrx to sink
    pub async fn room_to_sink(&mut self, mut sink: SplitSink<WebSocket, Message>) {
        let mut rrx = self.room.subscribe();
        let mut crx = self.close.subscribe();
        println!("D: HERE");
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Ok(rrx_msg) = rrx.recv() => {
                        println!("D: Room sent to Sink");
                        let _a = sink.send(Message::Text(rrx_msg)).await;
                    },
                    Ok(_) = crx.recv() => {
                        break;
                    }
                }
            }
            println!("D: OUT");
        });
    }

    // E: grx to rtx
    pub async fn game_to_room(&mut self) {
        let mut grx = self.game.subscribe();
        let rtx = self.room.clone();
        let mut crx = self.close.subscribe();
        if self.capacity > 1 {
            return;
        }
        println!("E: HERE");
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Ok(grx_msg) = grx.recv() => {
                        println!("E: Game sent to Room");
                        if let Err(e) = rtx.send(format!("{}", grx_msg)) {
                            println!("E: Breaking game_to_timer");
                            break;
                        }
                    },
                    Ok(_) = crx.recv() => {
                        break;
                    }
                }
            }
            println!("E: OUT");
        });
    }

    // F: trx to rtx
    pub async fn timer_to_room(&mut self) {
        let mut trx = self.timer.subscribe();
        let mut rtx = self.room.clone();
        let mut crx = self.close.subscribe();
        if self.capacity > 1 {
            return;
        }
        println!("F: HERE");
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Ok(msg) = trx.recv() => {
                        println!("F: Timer sent to Room");
                        for i in 0..5 {
                            let _ = rtx.send(format!("{}", i));
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        }
                    },
                    Ok(_) = crx.recv() => {
                        break;
                    }
                }
            }
        });
        
    }
}

impl GameState {
    pub async fn get_game_state(state: Arc<Mutex<RoomState>>, code: String) -> Self {
        let mut state_guard = state.lock().unwrap();
        let mut game_state = state_guard.state.entry(code.clone())
            .or_insert_with(|| GameState::default());
        game_state.capacity += 1;
        println!("GAME OBJECT {:?}", game_state);
        game_state.clone()
    }
}

#[derive(Debug, Clone)]
enum MatchState {
    Lobby,
    Pregame,
    Toss,
    FirstInning(usize),
    SecondInning(usize)
}