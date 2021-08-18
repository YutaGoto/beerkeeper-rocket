use rocket;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::models::response::Response;
use crate::services::welcome_service;

#[get("/", format = "json")]
pub fn welcome() -> status::Custom<Json<Response>> {
    let response = welcome_service::welcome();
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
