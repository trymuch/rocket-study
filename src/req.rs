#![allow(unused, dead_code)]
use std::path::PathBuf;

use rocket::{get, head, request::FromParam};
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

// 动态路径片段，作为请求处理函数的参数存在
// restful风格的请求路径
// 可以有多个动态路径片段
// 作为动态路径片段的参数都必须实现FromParam trait==》参数守卫
// 常见的实现FromParam的外部类型：数字类型、布尔类型、字符串切片、IP地址的类型
// 以上类型的Option、Result类型，并且总是请求总是成功返回的
#[get("/dyn/hello/<name>")]
pub fn hello(name: &str) -> String {
    format!("Hello, {}.", name)
}

#[get("/dyn/set/<flag>")]
pub fn set(flag: bool) -> String {
    format!("{}", flag)
}

#[get("/dyn/hello/<name>/<age>/<cool>")]
pub fn hello_name_age_cool(name: &str, age: u8, cool: bool) -> String {
    format!("{}--{}--{}", name, age, cool)
}

#[derive(Debug)]
pub(crate) struct MyParam<'r> {
    key: &'r str,
    value: usize,
}

impl<'r> FromParam<'r> for MyParam<'r> {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        let (key, val_str) = match param.find(":") {
            Some(i) if i > 0 => (&param[..i], &param[(i + 1)..]),
            _ => return Err(param),
        };
        if !key.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(param);
        }
        val_str
            .parse()
            .map(|value| MyParam { key, value })
            .map_err(|_| param)
    }
}

#[get("/dyn/definedParam/<param>")]
pub fn handle_defined_param(param: MyParam) -> String {
    format!("{:?}", param)
}

// 片段守卫，默认实现了FromSegment的类型是PathBuf
// 只有这一个类型，其他需要字节自定义，没有必要
#[get("/dyn/path/<path..>")]
pub fn get_path_buf(path: PathBuf) -> String {
    format!("{:?}", path)
}
