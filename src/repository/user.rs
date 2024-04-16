use crate::helpers::AuthError;
use crate::types::User;

use diesel::{ self, insert_into };
use crate::schema::users;

use argon2::{
    password_hash::{ rand_core::OsRng, PasswordHasher, SaltString },
    Argon2,
    PasswordHash,
};

use diesel::prelude::*;

#[tracing::instrument(name = "Register a new user", skip_all)]
pub async fn create_user(conn: &mut PgConnection, user: &User) -> Result<(), AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash_result = argon2.hash_password(user.hashed_password.as_bytes(), &salt)?;
    let password_hash_str = password_hash_result.to_string();

    let _password_hash = match PasswordHash::new(&password_hash_str) {
        Ok(hash) => hash,
        Err(err) => {
            return Err(AuthError::Argon2Error(err));
        }
    };

    match
        insert_into(users::table)
            .values((
                users::id.eq(&user.id),
                users::name.eq(&user.name),
                users::email.eq(&user.email),
                users::hashed_password.eq(&password_hash_str),
            ))
            .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(err) => Err(AuthError::DatabaseError(err)),
    }
}

#[tracing::instrument(name = "Fetch all registered users", skip_all)]
pub async fn fetch_all_users(
    conn: &mut PgConnection
    //  user_id: Uuid
) -> Result<Vec<User>, AuthError> {
    let user_list = users::table.load::<User>(conn).map_err(|err| AuthError::DatabaseError(err))?;

    Ok(user_list)
}
