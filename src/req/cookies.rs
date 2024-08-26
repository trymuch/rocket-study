use rocket::{
    get,
    http::{Cookie, CookieJar},
    serde::json::{serde_json::json, Value},
    time::Duration,
};

#[get("/cookies")]
pub fn index(cookies: &CookieJar) -> Value {
    println!("cookies: {:#?}", cookies);
    cookies.add(("first", "one"));
    let cookie = Cookie::build(("base", "12345"))
        .path("/")
        .domain("127.0.0.1")
        .secure(true)
        .max_age(Duration::seconds(30));
    cookies.add(cookie);
    cookies.add_private(("private_cookie1", "hello,world!"));
    cookies.add_private(("private_cookie2", "hello,rocket!"));
    println!("cookies after add: {:#?}", cookies);
    json!(
        {
            "cookies":"cookies",
        }
    )
}

#[get("/cookies/getAllCookies")]
pub fn get_all_cookies(jar: &CookieJar) {
    println!("cookies:{:#?}", jar);
}
