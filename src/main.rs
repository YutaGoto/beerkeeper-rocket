#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate bcrypt;
extern crate chrono;
extern crate dotenv;
extern crate jsonwebtoken;
extern crate serde;
extern crate serde_json;
extern crate uuid;

use dotenv::dotenv;

mod connection;
mod constants;
mod handlers;
mod jwt;
mod models;
mod schema;
mod services;

use crate::handlers::{events_handler, users_handler};

fn main() {
    dotenv().ok();
    rocket::ignite()
        .manage(connection::init_pool())
        .mount(
            "/events",
            routes![
                events_handler::create_event,
                events_handler::find_event,
                events_handler::participant,
                events_handler::delete_participant
            ],
        )
        .mount(
            "/users",
            routes![
                users_handler::login_user,
                users_handler::signup_user,
                users_handler::my_profile
            ],
        )
        .launch();
}
