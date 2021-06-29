use rocket::http::Status;

use crate::constants::messages_constant;
use crate::models::response::{Response, ResponseWithStatus};

pub fn not_found_response() -> ResponseWithStatus {
    ResponseWithStatus {
        status_code: Status::NotFound.code,
        response: Response {
            message: String::from(messages_constant::MESSAGE_NOT_FOUND),
            data: serde_json::to_value("").unwrap()
        },
    }
}
