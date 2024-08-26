#![allow(dead_code)]

use rocket::{fs::TempFile, get, head, post};
pub mod cookies;
pub mod dyn_path;
pub mod rank;
pub mod req_guard;
// 如果head请求有匹配的路由那么就由匹配的路由处理
// 但是如果没有head路由，那么head请求会匹配get路由
// 注意head请求的响应是没有body的
#[head("/handle_head_req")]
pub fn handle_head_req() -> &'static str {
    "我是处理head请求的!"
}

#[get("/handle_head_as_get")]
pub fn handle_head_as_get() -> &'static str {
    "将head请求当做get请求处理"
}

#[post("/upload", format = "plain", data = "<file>")]
pub async fn upload(mut file: TempFile<'_>) -> std::io::Result<()> {
    file.persist_to("/temp.txt").await
}
