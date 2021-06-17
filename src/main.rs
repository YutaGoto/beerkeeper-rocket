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
extern crate dotenv;
extern crate jsonwebtoken;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate uuid;

use dotenv::dotenv;

mod jwt;
mod schema;
mod connection;
mod constants;
mod handlers;
mod services;
mod models;
mod routers;

fn main() {
    dotenv().ok();
    routers::users_router::create_routes();
}
