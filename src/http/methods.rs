
// 封装get方法
pub fn get(url: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    println!("GET 方法");
    reqwest::blocking::get(url)
}