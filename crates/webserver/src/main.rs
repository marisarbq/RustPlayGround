use axum::{
    extract::{Path, Query},
    response::Html,
    routing::{get, post},
    Json, Router, Server,
};
use serde_json::{json, Value};
use std::collections::HashMap;

//因为我的IDE开发环境是在云端Docker容器，端口映射有一定规则， 容器 300x 端口会映射成外网1000x
static HOST: &str = "39.103.223.49";
static PORT_INDEX: &str = "9";

// #[] 是Rust attr属性，属性是作用在 Rust 语言元素上的元数据。
// https://cloud.tencent.com/developer/article/1594094
#[tokio::main]
async fn main() {
    // run it
    let true_host = format!("0.0.0.0:300{}", PORT_INDEX);
    let addr = true_host.parse().unwrap();
    println!(
        "[服务器已启动]
        地址:  {}
        公网:  {}:1000{}",
        addr, HOST, PORT_INDEX
    );
    Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "服务器在线!" }))
        .route("/user/:name", get(app_user))
        .route("/get", get(app_get))
        .route("/post", post(app_post))
}

//https://docs.rs/axum/latest/axum/#extractors
async fn app_user(
    Query(query): Query<HashMap<String, String>>, //query 和其他语言的web框架是一样的。
    Path(path): Path<String>,                     //获取url路径，也一样
) -> Html<&'static str> {
    print!(
        "
    query: {:#?}
    path: {:#?}
    ",
        query,
        path
    );
    Html("<h1>Hello, World!</h1>")
}

async fn app_get(
    Query(query): Query<HashMap<String, String>>, //query 和其他语言的web框架是一样的。
) -> Json<Value> {
    Json(json!({
        "code": 200,
        "success": true,
        "msg": "ok",
        "data": {
            "server_status": "online",
            "ip": HOST,
            "port_index": PORT_INDEX,
            "query": query
        }
    }))
}

async fn app_post(Json(payload): Json<Value>) -> Json<Value> {
    Json(json!({
        "code": 200,
        "success": true,
        "msg": "ok",
        "data": {
            "server_status": "online",
            "ip": HOST,
            "port_index": PORT_INDEX,
            "payload": payload
        }
    }))
}