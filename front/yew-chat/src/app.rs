use std::collections::HashMap;

use crate::{comp::TestClick, services::get};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;


static URL: &str = "https://httpbin.org/ip";

#[function_component(App)]
pub fn app() -> Html {

    let ans = use_state(|| format!("测试"));

    let anss = ans.clone();
    spawn_local(async move {
        let req = get(URL).await;
        println!("本机公网ip是： {:#?}", URL);
        match req {
            Err(err) => {
                println!("网络请求失败!,{}",err);
            }
            Ok(resp) => {
                let my_json = resp.json::<HashMap<String, String>>().await.ok().unwrap();
                match my_json.get_key_value("origin") {
                    None => {
                        println!("未获取到正确的IP");
                    }
                    Some(opt) => {
                        let a = format!("本机公网ip是： {}", opt.1.to_string());
                        anss.set(format!("请求完毕，{}",a));
                    }
                }
            }
        }
    });

    html! {
        <>
            <p>{format!("{}", *ans)}</p>
            <TestClick />
        </>
    }
}
