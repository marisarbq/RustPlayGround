use std::{rc::Rc, sync::mpsc::sync_channel};

use wasm_bindgen_futures::spawn_local;
use weblog::{console_log, web_sys::HtmlInputElement};
use yew::prelude::*;

use futures::{
    channel::mpsc::channel,
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use reqwasm::websocket::{futures::WebSocket, Message};

#[derive(Clone, PartialEq, Properties)]
pub struct ChatProps {
    #[prop_or_default]
    pub count: i64,
}

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

struct IOS {
    write: SplitSink<reqwasm::websocket::futures::WebSocket, Message>
}

#[function_component(Chat)]
pub fn chat_comp(props: &ChatProps) -> Html {
    let post = use_reducer(|| ChatState {
        msg: format!("null"),
    });

    let (mut tx, mut rx) = sync_channel::<String>(100);


    let mut sender = use_mut_ref(|| {
        let io = WebSocket::open(WS_URL).unwrap();
        let (mut write, mut read) =  io.split();

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                console_log!(format!("1. {:?}", msg))
            }
            console_log!("WebSocket Closed")
        });

        IOS {
            write
        }
    });


    // let ios = sender.borrow_mut();

    // let read = &ios.read;
    // let mut write = &ios.write;

    {
        let tx_init = tx.clone(); 



        use_effect_with_deps(
            move |_| {
                // * 卡在一个非常痛苦的问题上了，在yew的Hook当中如何持久化IO
                // https://stackoverflow.com/questions/67897874/rust-how-to-fix-borrowed-value-does-not-live-long-enough   

                spawn_local(async move {
                    tx_init.send(String::from("init"));
                    // 看来线程通信这东西我理解的还不够准确。
                    // while let Ok(msg) = rx.recv() {
                    //     console_log!("[rxinit]");
                    //     match opt {
                    //         Ok(msg) => {
                    //             console_log!("[rx]",&msg);
                    //             if msg.is_empty() {
                    //                 console_log!("[send_rx]",&msg);
                    //                 write.send(Message::Text(msg)).await;
                    //             }
                    //         }
                    //         Err(err) => {
                    //             console_log!("[rx_err]",format!("{:#?}",err));
                    //         }
                    //     }
                    //     // In any websocket error, break loop.
                    // }
                });

                
                || {}
            },
            (),
        );

        use_effect_with_deps(
            move |state| {
                if !(&state.msg.is_empty()) {
                    console_log!("[tx]", &state.msg);
                    tx.send(String::from(&state.msg));
                    
                    spawn_local(async move {
                        while let Ok(msg) = rx.recv() {
                            console_log!("[rx]", &msg);
                            if !msg.is_empty() {
                                console_log!("[send_rx]", &msg);
                                sender.borrow_mut().write.send(Message::Text(msg)).await;
                            }else {
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

    html! {
        <div>
            <input onchange={onchange} />
            <button onclick={onclick}>{"发送"}</button>
        </div>
    }
}
