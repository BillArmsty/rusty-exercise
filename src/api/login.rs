use actix_session::Session;
use actix_web::{ web, HttpResponse };
use argon2::{ Argon2, PasswordHash, PasswordVerifier };
use dotenvy::dotenv;
use serde_json::json;
use diesel::pg::{ Pg, PgConnection };
use diesel::sql_types::Text;
use diesel::prelude::*;
use secrecy::{ ExposeSecret, Secret };
use uuid::Uuid;
use diesel::result::Error as DieselError;

use crate::schema::users::dsl::*;
use crate::schema::users;

use crate::types::response::IdentityError;
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
                .insert("user_id", user_id.to_string())
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
) -> Result<Uuid, IdentityError> {
    let (user_id, password_hash) = get_stored_credentials(&credentials.email, connection).context(
        "Failed to get stored credentials"
    )?;

    verify_password(credentials.password, password_hash).context("Failed to verify password")?;

    user_id.ok_or_else(|| IdentityError::InvalidCredentials(anyhow::anyhow!("User not found")))
}

fn get_stored_credentials(
    email_param: &String,
    connection: &mut PgConnection
) -> Result<(Option<Uuid>, Secret<String>), anyhow::Error> {
    let result = users
        .filter(email.eq(email_param))
        .select((users::id, users::hashed_password))
        .first::<(Option<Uuid>, String)>(Text, connection)
        .optional()
        .map_err(|e| e.into());

    match result {
        Ok(Some((user_id, password_hash))) => {
            let secret_password_hash = Secret::new(password_hash);
            Ok((user_id, secret_password_hash))
        }
        Ok(None) | Err(DieselError::NotFound) => {
            let default_password_hash = Secret::new(
                "$argon2id$v=19$m=15000,t=2,p=1$\
            gZiV/M1gPc22ElAH/Jh1Hw$\
            CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno".to_string()
            );
            Ok((None, default_password_hash))
        }
        Err(err) => Err(err.into()),
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
