
use futures::{SinkExt, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;
use gloo_net::websocket::{Message, futures::WebSocket};

use crate::router::Route;

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    id: i32,
    name: String,
    price: i32
}
#[function_component]
pub fn Products() -> Html {

    // let mut ws = WebSocket::open("ws://localhost:8000/ws").unwrap();
    // let (mut write, mut read) = ws.split();

    let history = use_list(vec![]);
    let timer = use_state(|| 0);

    let ws = {
        let history = history.clone();
        let timer = timer.clone();
        use_websocket_with_options(
            format!("ws://localhost:8000/ws"),
            UseWebSocketOptions {
                // Receive message by callback `onmessage`.
                onmessage: Some(Box::new(move |message| {
                web_sys::console::log_1(&format!("ws [recv]: {}", message).into());

                    history.push(format!("ws [recv]: {}", message));
                    let num = message.parse::<i64>().unwrap();
                    timer.set(num);

                })),
                manual: Some(true),
                ..Default::default()
            },
        )
    };

    let onopen = {
        let ws = ws.clone();
        Callback::from(move |_| {
            ws.open();
        })
    };

    let onclose = {
        let ws = ws.clone();
        Callback::from(move |_| {
            ws.close();
        })
    };

    web_sys::console::log_1(&format!("Button clicked with value").into());


    html! {
        <div class="container">
        {
            "HEHE"
        }
        <button class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 bg-slate-900 text-slate-100 hover:bg-slate-900/90" onclick={onopen} disabled={*ws.ready_state != UseWebSocketReadyState::Closed}>{ "Connect to backend websocket" }</button>
        <button class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 bg-slate-900 text-slate-100 hover:bg-slate-900/90" onclick={onclose} disabled={*ws.ready_state != UseWebSocketReadyState::Open}>{ "Disconnect to backend websocket" }</button>
        {
            *timer        
        }

            // <h2>{"List of Products: "} {data.len()} </h2>
            // <p>{products}</p>
        </div>
    }
}