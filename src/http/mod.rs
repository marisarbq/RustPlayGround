use reqwest::Error;
use std::collections::HashMap;

use self::methods::get;
// 示范 http 请求&错误处理。
// 获取本机ip，并且考虑多种状况
// 熟悉语法
mod methods;

static URL: &str = "https://httpbin.org/ip";

pub fn request() -> Result<(), Error> {
    let req = get(URL);

    match req {
        Err(err) => {
            println!("网络请求失败!");
            Err(err)
        }
        Ok(resp) => {
            let resp = resp.json::<HashMap<String, String>>();
            match resp?.get_key_value("origin") {
                None => {
                    println!("未获取到正确的IP");
                }
                Some(opt) => {
                    println!("本机公网ip是： {:#?}", opt.1);
                }
            }
            Ok(())
        }
    }
}
