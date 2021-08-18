use dotenv::dotenv;

use crate::connection;
use crate::handlers::{events_handler, users_handler, welcome_handler};

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/", routes![welcome_handler::welcome])
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
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn hello_world() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
