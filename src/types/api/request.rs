use diesel::deserialize::Queryable;
use secrecy::Secret;
use serde::Deserialize;


#[derive(Deserialize, Queryable)]
pub struct CreateUser {
    pub name : String,
    pub email: String,
    pub password: Secret<String>,
    pub id: i32,
}

