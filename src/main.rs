#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;

mod schema;
mod connection;
mod routers;
mod handlers;
mod repositories;
mod models;

fn main() {
    dotenv().ok();
    routers::users_router::create_routes();
}
