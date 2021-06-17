use rocket;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::connection::DbConn;
use crate::models::response::Response;
use crate::models::user::{LoginDTO, UserDTO};
use crate::services::users_service;

#[post("/signup", format = "json", data = "<user>")]
pub fn signup_user(user: Json<UserDTO>, conn: DbConn) -> status::Custom<Json<Response>> {
    let response = users_service::signup(user.0, conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[post("/login", format = "json", data = "<login>")]
pub fn login_user(login: Json<LoginDTO>, conn: DbConn) -> status::Custom<Json<Response>> {
    let response = users_service::login(login.0, conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
