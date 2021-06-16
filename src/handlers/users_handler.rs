use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::models::user;
use crate::models::user::{User, NewUser, LoginUser, HeaderUser};
use crate::repositories::users_repository;

#[get("/")]
pub fn all_users(connection: DbConn) -> Result<Json<Vec<User>>, Status> {
    users_repository::show_users(&connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}

#[post("/", format ="application/json", data = "<new_user>")]
pub fn create_user(new_user: Json<NewUser>, connection: DbConn) ->  Result<status::Created<Json<User>>, Status> {
    println!("here 0 {}",&new_user.name);

    let new_user: NewUser = NewUser {
        name: new_user.name.clone(),
        email: new_user.email.clone(),
        password: user::hash_password(&new_user.password),
    };

    users_repository::create_user(new_user, &connection)
        .map(|user| user_created(user))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get_user(id: i32, connection: DbConn) -> Result<Json<User>, Status> {
    users_repository::get_user(id, &connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<user>")]
pub fn update_user(id: i32, user: Json<User>, connection: DbConn) -> Result<Json<User>, Status> {
    users_repository::update_user(id, user.into_inner(), &connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete_user(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    users_repository::delete_user(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

#[post("/login", format = "application/json", data = "<login_info>")]
pub fn login_user(login_info: Json<LoginUser>, connection: DbConn) -> Result<Json<HeaderUser>, Status> {
    let login_user: LoginUser = LoginUser {
        email: login_info.email.clone(),
        password: login_info.password.clone()
    };

    users_repository::login_user(login_user, &connection)
        .map(|user| {
            let header_user: HeaderUser = HeaderUser {
                user_id: user.id
            };
            Json(header_user)
        })
        .map_err(|error| error_status(error))
}

fn user_created(user: User) -> status::Created<Json<User>> {
    println!("here final");
    status::Created(
        format!("{host}:{port}/user/{id}", host = host(), port = port(), id = user.id).to_string(),
        Some(Json(user)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}
