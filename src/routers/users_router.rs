use rocket;
use crate::connection;
use crate::routers::users_router::rocket::routes;
use crate::handlers::users_handler;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount(
            "/users",
            routes![
                users_handler::login_user,
                users_handler::signup_user,
                users_handler::my_profile
            ],
        ).launch();
}
