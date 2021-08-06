use rocket;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket::http::Status;

use crate::jwt::UserToken;
use crate::connection::DbConn;
use crate::models::response::Response;
use crate::models::event::{EventDTO, Event};
use crate::models::user::User;
use crate::models::participantion::Participation;
use crate::services::{events_service, participantions_service, responses_services};

#[post("/", format = "json", data = "<ev>")]
pub fn create_event(token: Result<UserToken, status::Custom<Json<Response>>>, ev: Json<EventDTO>, conn: DbConn) -> status::Custom<Json<Response>> {
    match token {
        Ok(user_token) => {
            let user = User::find_user_by_email(&user_token.email, &conn).unwrap();
            let response = events_service::create(ev.0, user.id, conn);
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

#[post("/<id>/participant", format = "json")]
pub fn participant(id: i32, token: Result<UserToken, status::Custom<Json<Response>>>, conn: DbConn) -> status::Custom<Json<Response>> {
    match token {
        Ok(user_token) => {
            let user = User::find_user_by_email(&user_token.email, &conn).unwrap();
            let event_result = Event::find_by_id(id, &conn);
            if let Some(event) = event_result {
                let response = participantions_service::create(event.id, user.id, conn);
                status::Custom(
                    Status::from_code(response.status_code).unwrap(),
                    Json(response.response),
                )
            } else {
                let response = responses_services::not_found_response();
                status::Custom(
                    Status::from_code(response.status_code).unwrap(),
                    Json(response.response),
                )
            }
        },
        Err(e) => return e
    }
}

#[delete("/<id>/participant", format = "json")]
pub fn delete_participant(id: i32, token: Result<UserToken, status::Custom<Json<Response>>>, conn: DbConn) -> status::Custom<Json<Response>> {
    match token {
        Ok(user_token) => {
            let user = User::find_user_by_email(&user_token.email, &conn).unwrap();
            let event_result = Event::find_by_id(id, &conn);
            if let Some(event) = event_result {
                let option_participation = Participation::find_by_user_and_event(user.id, event.id, &conn);
                if let Some(participantion) = option_participation {
                    let response = participantions_service::delete(participantion.id, conn);
                    status::Custom(
                        Status::from_code(response.status_code).unwrap(),
                        Json(response.response),
                    )
                } else {
                    let response = responses_services::not_found_response();
                    status::Custom(
                        Status::from_code(response.status_code).unwrap(),
                        Json(response.response),
                    )
                }
            } else {
                let response = responses_services::not_found_response();
                status::Custom(
                    Status::from_code(response.status_code).unwrap(),
                    Json(response.response),
                )
            }
        },
        Err(e) => return e
    }
}
