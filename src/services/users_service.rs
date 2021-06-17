use rocket::http::Status;
use serde_json::json;

use crate::jwt;
use crate::connection::DbConn;
use crate::constants::messages_constant;
use crate::models::response::{Response, ResponseWithStatus};
use crate::models::user::{LoginDTO, User, UserDTO};

pub fn signup(user: UserDTO, conn: DbConn) -> ResponseWithStatus {
    match User::signup(user, &conn) {
        Ok(_) => {
            ResponseWithStatus {
                status_code: Status::Ok.code,
                response: Response {
                    message: String::from(messages_constant::MESSAGE_SIGNUP_SUCCESS),
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

pub fn login(login: LoginDTO, conn: DbConn) -> ResponseWithStatus {
    if let Some(result) = User::login(login, &conn) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(messages_constant::MESSAGE_LOGIN_SUCCESS),
                data: serde_json::to_value(json!({ "token": jwt::generate_token(result), "type": "Bearer" }))
                    .unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(messages_constant::MESSAGE_LOGIN_FAILED),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}