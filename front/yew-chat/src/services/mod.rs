use reqwest::Response;

// 封装get方法
pub async fn get(url: &str) -> Result<Response, reqwest::Error> {
    println!("GET 方法: {}", url);
    reqwest::get(url).await
}