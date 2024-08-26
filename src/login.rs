use rocket::{
    post,
    serde::{
        json::{serde_json::json, Json, Value},
        Deserialize, Serialize,
    },
};

use crate::jwt::get_jwt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct LoginUser {
    username: String,
    password: String,
}

#[post("/login", format = "application/json", data = "<login_user>")]
pub fn login(login_user: Json<LoginUser>) -> Result<Value, String> {
    println!("{}", login_user.username);
    println!("{}", login_user.password);
    let token = get_jwt()?;
    Ok(json!({
        "code":200,
        "message":"登录成功",
        "data":{
            "token":token,
        }
    }))
}
