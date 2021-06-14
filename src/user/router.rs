use rocket;
use crate::connection;
use crate::user;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount(
            "/users",
            routes![
                user::handler::all_users,
                user::handler::create_user,
                user::handler::get_user,
                user::handler::update_user,
                user::handler::delete_user,
                user::handler::login_user
            ],
        ).launch();
}
