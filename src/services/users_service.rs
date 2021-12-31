use rocket::http::Status;
use serde_json::json;

use crate::connection::DbConn;
use crate::constants::messages_constant;
use crate::jwt;
use crate::models::response::{Response, ResponseWithStatus};
use crate::models::user::{LoginDTO, User, UserDTO, UserInfo};

pub fn signup(user: UserDTO, conn: DbConn) -> ResponseWithStatus {
    match User::signup(user, &conn) {
        Ok(_) => ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(messages_constant::MESSAGE_SIGNUP_SUCCESS),
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

pub fn login(login: LoginDTO, conn: DbConn) -> ResponseWithStatus {
    if let Some(result) = User::login(login, &conn) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(messages_constant::MESSAGE_LOGIN_SUCCESS),
                data: serde_json::to_value(json!({
                    "id": result.id,
                    "token": jwt::generate_token(result),
                    "type": "Bearer"
                }))
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

pub fn find_by_email(email: &str, conn: DbConn) -> ResponseWithStatus {
    let option_user = User::find_user_by_email(email, &conn);
    if let Some(user) = option_user {
        let user_info: UserInfo = UserInfo {
            id: user.id,
            email: user.email,
            name: user.name,
        };
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(messages_constant::MESSAGE_OK),
                data: serde_json::to_value(user_info).unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::NotFound.code,
            response: Response {
                message: format!("user with email {} not found", email),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}
