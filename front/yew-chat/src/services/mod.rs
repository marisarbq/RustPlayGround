// 封装async get方法
pub async fn get(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    println!("GET 方法: {}", url);
    reqwest::get(url).await
}

pub async fn post(url: &str, body_data: String) -> Result<reqwest::Response, reqwest::Error> {
    println!("POST 方法: {}", url);
    reqwest::Client::new().post(url).body(body_data).send().await
}