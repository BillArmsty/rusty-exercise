use crate::{ helpers::AuthError, types::User };
use diesel::{ self, insert_into };
use crate::schema::users;

use argon2::{
    password_hash::{ rand_core::OsRng, PasswordHasher, SaltString },
    Argon2,
    PasswordHash,
};

use diesel::prelude::*;
pub async fn register_user(conn: &mut PgConnection, user: &User) -> Result<(), AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Hash the password
    let password_hash_result = argon2.hash_password(user.hashed_password.as_bytes(), &salt)?;
    let password_hash_str = password_hash_result.to_string();

    // Parse password hash string into PasswordHash struct
    let _password_hash = match PasswordHash::new(&password_hash_str) {
        Ok(hash) => hash,
        Err(err) => {
            return Err(AuthError::Argon2Error(err));
        }
    };

    // Insert user into the database
    match
        insert_into(users::table)
            .values((users::email.eq(&user.email), users::hashed_password.eq(&password_hash_str)))
            .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(err) => Err(AuthError::DatabaseError(err)),
    }
}
