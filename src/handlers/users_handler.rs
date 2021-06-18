use rocket;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::jwt::UserToken;
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

#[get("/profile", format = "json")]
pub fn my_profile(token: Result<UserToken, status::Custom<Json<Response>>>, conn: DbConn) -> status::Custom<Json<Response>> {
    match token {
        Ok(user_token) => {
            let response = users_service::find_by_email(&user_token.email, conn);
            status::Custom(
                Status::from_code(response.status_code).unwrap(),
                Json(response.response)
            )
        },
        Err(e) => return e
    }
}
