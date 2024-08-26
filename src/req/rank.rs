use rocket::get;

use super::req_guard::UserAuth;

#[get("/user/<id>")]
pub fn user(id: usize, _auth: UserAuth) -> String {
    format!("{}", id)
}

#[get("/user/<id>", rank = 2)]
pub fn user_int(id: isize, _auth: UserAuth) -> String {
    format!("{}", id)
}
#[get("/user/<id>", rank = 3)]
pub fn user_str(id: &str, _auth: UserAuth) -> String {
    format!("{}", id)
}
