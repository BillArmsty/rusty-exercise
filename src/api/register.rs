use actix_web::{ web, HttpResponse };
use secrecy::ExposeSecret;
use serde_json::json;
use crate::{ services::signup, types::User };

use diesel::prelude::*;

use crate::types::{ request::CreateUser, response::IdentityError };

pub async fn register(
    conn: &mut PgConnection,
    form: web::Form<CreateUser>
) -> Result<HttpResponse, IdentityError> {
    let user: CreateUser = form.into_inner();

    signup(
        conn,
        User {
            email: user.email,
            name: user.name,
            hashed_password: user.password.expose_secret().clone(),
            id: user.id,
        }).await;
    Ok(
        HttpResponse::Ok().json(
            json!({
        "msg": "User created successfully",
        "success": true,
        })
        )
    )
}
