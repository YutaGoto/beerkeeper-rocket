use rocket;
use crate::connection;
use crate::routers::events_router::rocket::routes;
use crate::handlers::events_handler;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount(
            "/users",
            routes![
                events_handler::create_event,
                events_handler::find_event
            ],
        ).launch();
}
