use rocket::http::Status;
use serde_json;

use crate::connection::DbConn;
use crate::constants::messages_constant;
use crate::models::participantion::Participation;
use crate::models::response::{Response, ResponseWithStatus};

pub fn create(event_id: i32, user_id: i32, conn: DbConn) -> ResponseWithStatus {
    match Participation::create(user_id, event_id, &conn) {
        Ok(_) => ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(messages_constant::MESSAGE_OK),
                data: serde_json::to_value("").unwrap(),
            },
        },
        Err(e) => ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: e.to_string(),
                data: serde_json::to_value("").unwrap(),
            },
        },
    }
}

pub fn delete(paticipation_id: i32, conn: DbConn) -> ResponseWithStatus {
    match Participation::delete(paticipation_id, &conn) {
        Ok(_) => ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(messages_constant::MESSAGE_OK),
                data: serde_json::to_value("").unwrap(),
            },
        },
        Err(e) => ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: e.to_string(),
                data: serde_json::to_value("").unwrap(),
            },
        },
    }
}
