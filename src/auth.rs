use argon2::{
    password_hash::{ rand_core::OsRng,  PasswordHash, PasswordHasher, PasswordVerifier, SaltString },
    Argon2,
};

use diesel::prelude::*;
use diesel::{ self, insert_into };

use crate::schema::users;

#[derive(Debug)]
pub enum AuthError {
    IncorrectPassword,
    NoEmailSet,
    NoNameSet,
    NoPasswordSet,
    EnvironmentError(dotenvy::Error),
    Argon2Error(argon2::password_hash::Error),
    DatabaseError(diesel::result::Error),
}

impl From<argon2::password_hash::Error> for AuthError {
    fn from(e: argon2::password_hash::Error) -> Self {
        AuthError::Argon2Error(e)
    }
}

pub use self::AuthError::{ IncorrectPassword, NoEmailSet, NoNameSet, NoPasswordSet };

#[derive(Queryable, Identifiable, Debug, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub email: String,
}

#[derive(Queryable)]
pub struct UserWithPassword {
    user: User,
    password: String,
}

pub fn current_user_from_env(conn: &mut PgConnection) -> Result<Option<User>, AuthError> {
    let email = get_email()?;
    let password = get_password()?;
    find_user(conn, &email, &password)
}

pub fn register_user_from_env(conn: &mut PgConnection) -> Result<User, AuthError> {
    let email = get_email()?;
    let password = get_password()?;
    register_user(conn, &email, &password)
}

fn find_user(
    conn: &mut PgConnection,
    email: &str,
    password: &str
) -> Result<Option<User>, AuthError> {
    let user_and_password = users::table
        .filter(users::email.eq(email))
        .select(((users::id, users::email), users::hashed_password))
        .first::<UserWithPassword>(conn)
        .optional()
        .map_err(AuthError::DatabaseError)?;

    if let Some(user_and_password) = user_and_password {
        let parsed_hash = PasswordHash::new(&user_and_password.password)?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|e| {
                match e {
                    argon2::password_hash::Error::Password => IncorrectPassword,
                    _ => AuthError::Argon2Error(e),
                }
            })?;
        Ok(Some(user_and_password.user))
    } else {
        Ok(None)
    }
}

fn register_user(
    conn: &mut PgConnection,
    email: &str,
    password: &str
) -> Result<User, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    insert_into(users::table)
        .values((users::email.eq(email), users::hashed_password.eq(hashed_password)))
        .returning((users::id, users::email))
        .get_result(conn)
        .map_err(AuthError::DatabaseError)
}

fn get_email() -> Result<String, AuthError> {
    if_not_present(dotenvy::var("EMAIL"), NoEmailSet)
}

fn get_password() -> Result<String, AuthError> {
    if_not_present(dotenvy::var("PASSWORD"), NoPasswordSet)
}

fn if_not_present<T>(
    res: Result<T, dotenvy::Error>,
    on_not_present: AuthError
) -> Result<T, AuthError> {
    use std::env::VarError::NotPresent;

    res.map_err(|e| {
        match e {
            dotenvy::Error::EnvVar(NotPresent) => on_not_present,
            e => AuthError::EnvironmentError(e),
        }
    })
}
