use std::{rc::Rc, sync::mpsc::sync_channel};

use wasm_bindgen_futures::spawn_local;
use weblog::{console_log, web_sys::{HtmlInputElement, Text}};
use yew::prelude::*;

use futures::{
    channel::mpsc::channel,
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use reqwasm::websocket::{futures::WebSocket, Message};

// #[derive(Clone, PartialEq, Properties)]
// pub struct ChatProps {
//     #[prop_or_default]
//     pub count: i64,
// }

static WS_URL: &str = "ws://39.103.223.49:10009/websocket";

#[derive(Clone, Debug, PartialEq)]
struct ChatState {
    msg: String,
}

enum ActionName {
    Initial,
    Set,
    Send,
}

struct PayAction<T> {
    kind: ActionName,
    payload: T,
}

impl Reducible for ChatState {
    type Action = PayAction<String>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        ChatState {
            msg: action.payload,
        }
        .into()
    }
}

// impl Reducible for IOState {
//     type Action = PayAction<String>;

//     fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
//         Self {
//             io: self.io
//         }.into()
//     }
// }

struct IOHandler {
    emit: SplitSink<reqwasm::websocket::futures::WebSocket, Message>,
}

#[function_component(Chat)]
pub fn chat_comp() -> Html {
    let post = use_reducer(|| ChatState {
        msg: format!("null"),
    });

    let (tx, rx) = sync_channel::<String>(100);

    let history = use_state(|| {
        let stack = vec![];
        stack
    });

    let render_history = history.clone();

    let sender = use_mut_ref(|| {
        let io = WebSocket::open(WS_URL).unwrap();
        let (write, mut read) = io.split();

        let _his = history.clone();
        
        spawn_local(async move {
            while let Some(msg) = read.next().await {
                let mut messages = (*history).clone();
                match msg {
                    Ok(str) => {
                        console_log!(format!("messages:{:#?}", history));
                        messages.push(format!("{:?}", str));
                        _his.set(messages);
                        //关于Rust 局部变量带走的问题
                        //https://zhuanlan.zhihu.com/p/109285917
                        // let outstr = format!("{:#?}",msg);
                    }
                    Err(err) => {}
                }
            }
            console_log!("WebSocket Closed")
        });

        IOHandler { emit: write }
    });

    // console_log!(format!("{:#?}", history));

    // let ios = sender.borrow_mut();

    // let read = &ios.read;
    // let mut write = &ios.write;

    {
        let tx_init = tx.clone();

        use_effect_with_deps(
            move |_| {
                // * 卡在一个非常痛苦的问题上了, 已经解决了。菜就是原罪
                // https://stackoverflow.com/questions/67897874/rust-how-to-fix-borrowed-value-does-not-live-long-enough

                spawn_local(async move {
                    let _ = tx_init.send(String::from("init"));
                });

                || {}
            },
            (),
        );

        use_effect_with_deps(
            move |state| {
                if !(&state.msg.is_empty()) {
                    console_log!("[tx]", &state.msg);
                    let _ = tx.send(String::from(&state.msg));
                    spawn_local(async move {
                        while let Ok(msg) = rx.recv() {
                            console_log!("[rx]", &msg);
                            if !msg.is_empty() {
                                console_log!("[send_rx]", &msg);
                                let _ = sender.borrow_mut().emit.send(Message::Text(msg)).await;
                            } else {
                                console_log!("[rx_empty]")
                            }
                        }
                    });
                }
                || {}
            },
            post.clone(),
        );

        console_log!("[Update]执行1");
    }

    let onclick = {
        // let post = post.clone();
        //https://stackoverflow.com/questions/30177395/when-does-a-closure-implement-fn-fnmut-and-fnonce
        Callback::from(move |_| {})
    };

    //关于异步：https://learnku.com/articles/45127

    let onchange = {
        let post = post.clone();
        Callback::from(move |e: html::onchange::Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            post.dispatch(PayAction {
                kind: ActionName::Set,
                payload: input.value(),
            });
            console_log!(input.value());
            input.set_value("");
        })
    };

    let key = render_history.iter().map(|x| {
        html! {
            <p>{x}</p>
        }
    });

    html! {
        <div>
            <div>
                {for key}
            </div>
            <input onchange={onchange} />
            <button onclick={onclick}>{"发送"}</button>
        </div>
    }
}
