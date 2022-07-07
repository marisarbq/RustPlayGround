use crate::{comp::TestClick, services::get};
use crate::comp::chat::Chat;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

// static URL: &str = "https://httpbin.org/ip";
static URL: &str = "http://39.103.223.49:10009/get";

#[function_component(App)]
pub fn app() -> Html {
    let ans = use_state(|| format!("测试"));

    {
        let anss = ans.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                let req = get(URL).await;
                match req {
                    Err(err) => {
                        anss.set(format!("网络请求失败!,{}", err));
                    }
                    Ok(resp) => {
                        // anss.set(format!("请求结果: {:#?}", resp));
                        let my_json =
                            resp.json::<Value>().await.ok().unwrap();
                        anss.set(format!("请求结果: {:#?}", my_json));
                        // 测试
                        // match my_json.get_key_value("origin") {
                        //     None => {
                        //         println!("未获取到正确的IP");
                        //     }
                        //     Some(opt) => {
                        //         let a = format!("本机公网ip是： {}", opt.1.to_string());
                        //         anss.set(format!("请求完毕，{}",a));
                        //     }
                        // }
                    }
                }
            });
            || () 
            },
            (),
        );
    }

    html! {
        <>
            <p>{format!("{}", *ans)}</p>
            <TestClick />
            <Chat />
        </>
    }
}
