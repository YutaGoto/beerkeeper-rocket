#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::models::user;
use crate::models::user::{User, NewUser, LoginUser};

use crate::schema::users;
use crate::schema::users::dsl::*;

pub fn create_user(new_user: NewUser, conn: &PgConnection) -> QueryResult<User> {
    diesel::insert_into(users::table).values(&new_user).get_result(conn)
}

pub fn show_users(connection: &PgConnection) -> QueryResult<Vec<User>>  {
    //users.filter(published.eq(true))
    users.limit(5).load::<User>(&*connection)
}

pub fn get_user(user_id: i32, connection: &PgConnection) -> QueryResult<User> {
    users::table.find(user_id).get_result::<User>(connection)
}

pub fn update_user(user_id: i32, user: User, connection: &PgConnection) -> QueryResult<User> {
    diesel::update(users::table.find(user_id)).set(&user).get_result(connection)
}

pub fn delete_user(user_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(users::table.find(user_id)).execute(connection)
}

pub fn login_user(login_user: LoginUser, connection: &PgConnection) -> QueryResult<User> {
    let hash_password = user::hash_password(&login_user.password);
    users::table.filter(users::email.eq(login_user.email)).filter(users::password.eq(hash_password)).first::<User>(connection)
}
