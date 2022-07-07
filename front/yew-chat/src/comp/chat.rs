use std::fmt::format;

use reqwasm::websocket::{futures::{WebSocket}, Message};
use weblog::console_log;
use yew::prelude::*;

// use gloo_net::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures::{SinkExt, StreamExt};

#[derive(Clone, PartialEq, Properties)]
pub struct ChatProps {
    #[prop_or_default]
    pub count: i64,
}

static WS_URL: &str = "ws://39.103.223.49:10009/websocket";


#[function_component(Chat)]
pub fn chat_comp(props: &ChatProps) -> Html {
    let ChatProps { count } = props;

    let msg = use_state(|| format!("null"));
    
    console_log!("dida1");

    let ws = WebSocket::open(WS_URL).unwrap();
    let (mut write, mut read) = ws.split();

    spawn_local(async move {
        console_log!("dida2");
        write
            .send(Message::Text(String::from("test")))
            .await
            .unwrap();
        write
            .send(Message::Text(String::from("test 2")))
            .await
            .unwrap();
    });

    spawn_local(async move {
        while let Some(msg) = read.next().await {
            console_log!(format!("1. {:?}", msg))
        }
        console_log!("WebSocket Closed")
    });

    html! {
        <div>
            <input />
            <button onclick={Callback::from(|_| {
                console_log!("Click!");
            })}>{"发送"}</button>
        </div>
    }
}
