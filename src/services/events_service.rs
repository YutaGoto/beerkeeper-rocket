use rocket::http::Status;
use serde_json;

use crate::connection::DbConn;
use crate::constants::messages_constant;
use crate::models::event::{Event, EventDTO};
use crate::models::response::{Response, ResponseWithStatus};

pub fn create(event: EventDTO, user_id: i32, conn: DbConn) -> ResponseWithStatus {
    let option_event = Event::create(event, user_id, &conn);
    if let Some(e) = option_event {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(messages_constant::MESSAGE_OK),
                data: serde_json::to_value(e).unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(messages_constant::MESSAGE_CAN_NOT_INSERT_DATA),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn find_by_id(i: &i32, conn: DbConn) -> ResponseWithStatus {
    let option_event = Event::find_by_id(*i, &conn);
    if let Some(event) = option_event {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(messages_constant::MESSAGE_OK),
                data: serde_json::to_value(event).unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::NotFound.code,
            response: Response {
                message: String::from(messages_constant::MESSAGE_NOT_FOUND),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}
