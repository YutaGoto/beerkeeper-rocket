#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::users;

use crypto::digest::Digest;
use crypto::sha3::Sha3;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub login_session: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub login_session: String,
}

#[derive(Queryable, Serialize)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct HeaderUser {
    pub user_id: i32,
    pub login_session: String,
}

pub fn hash_password(password: &String) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(password);
    hasher.result_str()
}
