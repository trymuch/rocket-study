use rocket::{
    get,
    http::Status,
    request::{FromRequest, Outcome},
    serde::{
        json::{serde_json::json, Value},
        Serialize,
    },
    Request,
};

use crate::jwt::validate;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct AdminAuth<'r>(&'r str);

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct UserAuth<'r>(&'r str);
#[derive(Debug)]
pub(crate) enum AuthError {
    Missing,
    Invalid,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminAuth<'r> {
    type Error = AuthError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let header = request.headers().get_one("Authorization");
        let token = match header {
            None => return Outcome::Error((Status::BadRequest, AuthError::Missing)),
            Some(token) => token,
        };
        match validate(token) {
            Some(token_data) if token_data.claims.permission == "admin" => {
                Outcome::Success(AdminAuth(token))
            }
            _ => Outcome::Error((Status::Unauthorized, AuthError::Invalid)),
        }
    }
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAuth<'r> {
    type Error = AuthError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let header = request.headers().get_one("Authorization");
        let token = match header {
            None => return Outcome::Error((Status::BadRequest, AuthError::Missing)),
            Some(token) => token,
        };
        match validate(token) {
            Some(token_data) if token_data.claims.permission == "user" => {
                Outcome::Success(Self(token))
            }
            _ => Outcome::Error((Status::Unauthorized, AuthError::Invalid)),
        }
    }
}
#[get("/req_guard")]
pub fn req_guard(auth: AdminAuth) -> Value {
    json!(
        {"auth":auth}
    )
}

#[get("/admin")]
pub fn admin(auth: AdminAuth) -> Value {
    json!({
        "auth":auth,
    })
}

#[get("/user")]
pub fn user(auth: UserAuth) -> Value {
    json!({
        "auth":auth,
    })
}
