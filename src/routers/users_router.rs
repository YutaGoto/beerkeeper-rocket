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
                users_handler::all_users,
                users_handler::create_user,
                users_handler::get_user,
                users_handler::update_user,
                users_handler::delete_user,
                users_handler::login_user
            ],
        ).launch();
}
