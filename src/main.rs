#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
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
use rocket::http::Method;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000",
        "http://127.0.0.1:3000",
        "http://localhost:8000",
        "http://0.0.0.0:8000",
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Put, Method::Post, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
            "Content-Type",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .attach(make_cors())
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
