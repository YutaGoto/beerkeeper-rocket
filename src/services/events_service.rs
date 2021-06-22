use rocket::http::Status;
use serde_json;

use crate::connection::DbConn;
use crate::constants::messages_constant;
use crate::models::response::{Response, ResponseWithStatus};
use crate::models::event::{Event, EventDTO};

pub fn create(event: EventDTO, user_id: i32, conn: DbConn) -> ResponseWithStatus {
    match Event::create(event, user_id, &conn) {
        Ok(_) => {
            ResponseWithStatus {
                status_code: Status::Ok.code,
                response: Response {
                    message: String::from(messages_constant::MESSAGE_OK),
                    data: serde_json::to_value("").unwrap(),
                },
            }
        },
        Err(e) => {
            ResponseWithStatus {
                status_code: Status::BadRequest.code,
                response: Response {
                    message: e.to_string(),
                    data: serde_json::to_value("").unwrap(),
                },
            }
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
