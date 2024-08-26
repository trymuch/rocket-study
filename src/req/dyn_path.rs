use std::path::PathBuf;

use rocket::{get, request::FromParam};

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
// 片段守卫必须是路径的最后一部分
// "/dyn/path/<path..>/other" 这种路径是错的
#[get("/dyn/path/<path..>")]
pub fn get_path_buf(path: PathBuf) -> String {
    format!("{:?}", path)
}

// 忽略的片段
#[get("/dyn/ignore/<_>/ignore")]
pub fn ignore_segment() -> &'static str {
    "<_>在路径中是可以忽略的片段，不会出现在的请求处理函数的参数列表"
}
// 被忽略的多个片段后面不能有其他的片段
// 如果写了多余的片段会被忽略
#[get("/dyn/ignore_multi/<_..>")]
pub fn ignore_multiple_segment() -> &'static str {
    "可以忽略多个片段"
}
