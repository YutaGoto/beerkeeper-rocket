use rocket::http::Status;
use serde_json::json;

use crate::constants::messages_constant;
use crate::models::response::{Response, ResponseWithStatus};

pub fn welcome() -> ResponseWithStatus {
    ResponseWithStatus {
        status_code: Status::Ok.code,
        response: Response {
            message: String::from(messages_constant::MESSAGE_OK),
            data: serde_json::to_value(json!({"value" : "Welcome beerkeeper"})).unwrap(),
        },
    }
}
