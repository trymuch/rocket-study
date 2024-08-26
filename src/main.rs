use rocket::{get, launch, routes};

mod jwt;
mod login;
mod req;

/// launch 是一个过程宏
/// launch 生成一个运行Rocket服务器对象的的main函数
/// launch 注解的函数返回一个Rocket<Build>对象，可以让rust自动推断，不必自己写出
/// launch 自动初始化异步运行时，并运行Rocket服务器实例
#[launch]
fn rocket() -> _ {
    // build函数构建一个Rocket实例
    // Rocket类型使用构建器模式编写，可以链式调用
    rocket::build()
        // 装载路由：将用户访问的请求和处理程序映射起来
        // 用户的请求匹配了相应的路由就可以调用对应的处理程序
        .mount("/", routes![index, login::login])
        .mount(
            "/req",
            routes![
                req::handle_head_req,
                req::handle_head_as_get,
                req::dyn_path::hello,
                req::dyn_path::set,
                req::dyn_path::hello_name_age_cool,
                req::dyn_path::handle_defined_param,
                req::dyn_path::get_path_buf,
                req::dyn_path::ignore_segment,
                req::dyn_path::ignore_multiple_segment,
                req::req_guard::req_guard,
                req::req_guard::admin,
                req::req_guard::user,
                req::cookies::index,
                req::cookies::get_all_cookies,
                req::upload,
            ],
        )
        .mount(
            "/req/rank",
            routes![req::rank::user, req::rank::user_int, req::rank::user_str,],
        )
}

#[get("/")]
fn index() -> &'static str {
    "Hello,world!"
}
