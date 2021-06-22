use rocket;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::jwt::UserToken;
use crate::connection::DbConn;
use crate::models::response::Response;
use crate::models::event::EventDTO;
use crate::models::user::User;
use crate::services::events_service;

#[post("/", format = "json", data = "<e>")]
pub fn create_event(token: Result<UserToken, status::Custom<Json<Response>>>, e: EventDTO, conn: DbConn) -> status::Custom<Json<Response>> {
    match token {
        Ok(user_token) => {
            let user = User::find_user_by_email(&user_token.email, &conn).unwrap();
            let response = events_service::create(e, user.id, conn);
            status::Custom(
                Status::from_code(response.status_code).unwrap(),
                Json(response.response),
            )
        },
        Err(e) => return e
    }
}


#[get("/<id>", format = "json")]
pub fn find_event(id: i32, token: Result<UserToken, status::Custom<Json<Response>>>, conn: DbConn) -> status::Custom<Json<Response>> {
    match token {
        Ok(_) => {
            let response = events_service::find_by_id(&id, conn);
            status::Custom(
                Status::from_code(response.status_code).unwrap(),
                Json(response.response)
            )
        },
        Err(e) => return e
    }
}
