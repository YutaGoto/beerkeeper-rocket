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
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .attach(CORS)
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
