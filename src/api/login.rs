use actix_session::Session;
use actix_web::{ web, HttpResponse };
use argon2::{ Argon2, PasswordHash, PasswordVerifier };
use diesel::{
    // query_builder::QueryId,
    //  SqlType,
    //  sql_types::Uuid,
    Identifiable,
};
use serde::Serialize;
use serde_json::json;
use diesel::prelude::*;
use secrecy::{ ExposeSecret, Secret };
use uuid::Uuid;

use diesel::result::Error as DieselError;

use crate::schema::users::dsl::*;
use crate::schema::users;

use crate::types::response::IdentityError;
use crate::types::{ PgPool, PooledConnection };
use anyhow::Context;
use std::fmt;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    password: Secret<String>,
}

pub struct Credentials {
    pub email: String,
    pub password: Secret<String>,
}

#[derive(Queryable, Identifiable, Debug, PartialEq, Eq, Clone, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User(id={}, email={})", self.id, self.email)
    }
}

pub fn check_auth(session: &Session) -> Result<Uuid, IdentityError> {
    let user_id = session
        .get::<Uuid>("user_id")
        .map_err(|e| IdentityError::InvalidCredentials(e.into()))?;

    match user_id {
        Some(user_id) => Ok(user_id),
        None => Err(IdentityError::InvalidCredentials(anyhow::anyhow!("User not authenticated"))),
    }
}

#[tracing::instrument(
    name = "Logging in a user", 
    skip(form, pool, session),
    fields(
        email = %form.email,
        user_id = tracing::field::Empty,
    )
)]
pub async fn login(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    session: Session
) -> Result<HttpResponse, IdentityError> {
    let credentials = Credentials {
        email: form.0.email,
        password: form.0.password,
    };

    let mut connection = pool.get().unwrap();

    match validate_credentials(credentials, &mut connection).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", tracing::field::display(&user_id));

            session.renew();
            session
                .insert("user_id", user_id)
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
        Err(e) => {
            tracing::event!(tracing::Level::ERROR, "Failed to login user: {:?}", e);
            Err(e)
        }
    }
}

#[tracing::instrument(name = "Validating user credentials", skip(credentials, connection))]
async fn validate_credentials(
    credentials: Credentials,
    connection: &mut PooledConnection
) -> Result<Uuid, IdentityError> {
    let (user_id, password_hash) = get_stored_credentials(
        &credentials.email,
        connection
    ).await.context("Failed to get stored credentials")?;

    let current_span = tracing::Span::current();

    tokio::task
        ::spawn_blocking(move || {
            current_span.in_scope(|| verify_password(credentials.password, password_hash))
        }).await
        .context("Failed to sppawn blocking thread")??;
    user_id
        .ok_or_else(|| anyhow::anyhow!("User not found"))
        .map_err(IdentityError::InvalidCredentials)
}

async fn get_stored_credentials(
    email_param: &String,
    connection: &mut PooledConnection
) -> Result<(Option<Uuid>, Secret<String>), anyhow::Error> {
    let result = users::table
        .filter(email.eq(email_param))
        .select((users::id, users::hashed_password))
        .first::<(Uuid, String)>(connection)
        .optional()
        .map_err(|e| e.into());

    match result {
        Ok(Some((user_id, password_hash))) => Ok((Some(user_id), Secret::new(password_hash))),
        Ok(None) => Ok((None, Secret::new("".to_string()))),
        Err(DieselError::NotFound) => Ok((None, Secret::new("".to_string()))),
        Err(e) => Err(e.into()),
    }
}

fn verify_password(
    password: Secret<String>,
    password_hash: Secret<String>
) -> Result<(), anyhow::Error> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&password_hash.expose_secret()).expect(
        "Failed to parse password hash"
    );

    argon2
        .verify_password(password.expose_secret().as_bytes(), &parsed_hash)
        .map_err(|_| anyhow::anyhow!("Invalid password"))
}

#[tracing::instrument(name = "Logging out a user", skip(session))]
pub async fn logout_user(session: Session) -> HttpResponse {
    if check_auth(&session).is_err() {
        return HttpResponse::Unauthorized().body("User not authenticated");
    }
    session.purge();
    HttpResponse::SeeOther()
        .append_header(("Location", "/login"))
        .body("User logged out successfully.")
}
