// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]


use chrono::NaiveDateTime;
use uuid::Uuid;
#[derive(Queryable, Debug)]
pub struct Confirmation {
    pub id: Uuid,
    pub email: String,
    pub expires_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Userr {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub hashed_password: String,
}
