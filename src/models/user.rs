use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

use crate::jwt::UserToken;
use crate::schema::users;
use crate::schema::users::dsl::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    pub login_session: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserDTO {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct LoginInfoDTO {
    pub id: i32,
    pub email: String,
    pub login_session: String,
}

impl User {
    pub fn signup(user: UserDTO, conn: &PgConnection) -> QueryResult<usize> {
        let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
        let user = UserDTO {
            password: hashed_pwd,
            ..user
        };

        diesel::insert_into(users).values(&user).execute(conn)
    }

    pub fn login(login: LoginDTO, conn: &PgConnection) -> Option<LoginInfoDTO> {
        let user_to_verify = users
            .filter(email.eq(&login.email))
            .get_result::<User>(conn);
        if let Ok(user) = user_to_verify {
            if !user.password.is_empty() && verify(&login.password, &user.password).unwrap() {
                let login_session_str = User::generate_login_session();
                User::update_login_session_to_db(&user.email, &login_session_str, conn);
                Some(LoginInfoDTO {
                    id: user.id,
                    email: user.email,
                    login_session: login_session_str,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn is_valid_login_session(user_token: &UserToken, conn: &PgConnection) -> bool {
        users
            .filter(email.eq(&user_token.email))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn)
            .is_ok()
    }

    pub fn find_user_by_email(un: &str, conn: &PgConnection) -> Option<User> {
        let result_user = users.filter(email.eq(un)).get_result::<User>(conn);
        if let Ok(user) = result_user {
            Some(user)
        } else {
            None
        }
    }

    pub fn find_by_id(i: i32, conn: &PgConnection) -> Option<User> {
        let result_user = users.find(i).get_result::<User>(conn);
        if let Ok(user) = result_user {
            Some(user)
        } else {
            None
        }
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn update_login_session_to_db(
        un: &str,
        login_session_str: &str,
        conn: &PgConnection,
    ) -> bool {
        if let Some(user) = User::find_user_by_email(un, conn) {
            diesel::update(users.find(user.id))
                .set(login_session.eq(login_session_str.to_string()))
                .execute(conn)
                .is_ok()
        } else {
            false
        }
    }
}
