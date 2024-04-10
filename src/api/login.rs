use actix_session::Session;
use actix_web::{ web, HttpResponse };
use argon2::{ Argon2, PasswordHash, PasswordVerifier };
use dotenvy::dotenv;
use serde_json::json;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use secrecy::{ ExposeSecret, Secret };
// use uuid::Uuid;
use diesel::result::Error as DieselError;

use crate::schema::users::dsl::*;
use crate::schema::users;

use crate::types::response::IdentityError;
// use crate::types::User;
use anyhow::Context;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    password: Secret<String>,
}

pub struct Credentials {
    pub email: String,
    pub password: Secret<String>,
}

#[derive(Queryable, Identifiable, Debug, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub email: String,
}



pub async fn login(
    form: web::Form<FormData>,
    pool: &mut PgConnection,
    session: Session
) -> Result<HttpResponse, IdentityError> {
    let credentials = Credentials {
        email: form.email.clone(),
        password: form.password.clone(),
    };

    match validate_credentials(credentials, pool).await {
        Ok(user_id) => {
            session.renew();
            session
                .insert("user_id", user_id.id.to_string())
                .map_err(|e| IdentityError::UnexpectedError(e.into()))?;

            Ok(
                HttpResponse::Ok().json(
                    json!({
                "success": true,
                "msg": "Login successful"
            })
                )
            )
        }
        Err(e) => Err(e),
    }
}

async fn validate_credentials(
    credentials: Credentials,
    connection: &mut PgConnection
) -> Result<User, IdentityError> {
    
    let stored_credentials = get_stored_credentials(&credentials.email, connection)
        .context("Failed to get stored credentials")?;

    //use stored_credentials to check if the password is correct
    match stored_credentials {
        Some((_user_id, password_hash)) => {
            verify_password(credentials.password, Secret::new(password_hash))
                .context("Failed to verify password")?;
            let user = users::table
                .filter(email.eq(&credentials.email))
                .select((users::id, users::email))
                .first::<User>(connection)
                .context("Failed to get user")
                .map_err(|e| IdentityError::UnexpectedError(e.into()))?;
            Ok(user)
        }
        None => Err(IdentityError::InvalidCredentials(
            anyhow::anyhow!("Invalid credentials")
        )),
    }
  
}


fn get_stored_credentials(
    email_param: &String,
    connection: &mut PgConnection
) -> Result<Option<(i32, String)>, anyhow::Error> {
    let result = users::table
        .filter(email.eq(email_param))
        .select((users::id, users::hashed_password))
        .first::<(i32, String)>(connection)
        .optional()
        .map_err(|e| e.into());
    
    match result {
        Ok(Some((user_id, password))) => Ok(Some((user_id, password))),
        Ok(None) => Ok(None),
        Err(DieselError::NotFound) => Ok(None),
        Err(e) => Err(e.into()),
    }
}


fn verify_password(
    password: Secret<String>,
    password_hash: Secret<String>
) -> Result<(), anyhow::Error> {
    dotenv().ok();

    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&password_hash.expose_secret()).expect(
        "Failed to parse password hash"
    );

    argon2
        .verify_password(password.expose_secret().as_bytes(), &parsed_hash)
        .map_err(|_| anyhow::anyhow!("Invalid password"))
}
