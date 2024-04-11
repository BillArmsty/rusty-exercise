use actix_web::{ web, HttpResponse };
use anyhow::Context;
use serde_json::json;
use crate::{ services::signup, types::{PgPool, User} };


use crate::types::{ request::CreateUser, response::IdentityError };

pub async fn register(
    pool: web::Data<PgPool>,
    form: web::Form<CreateUser>
) -> Result<HttpResponse, IdentityError> {

    let user: User = form.into_inner().try_into()?;


    let mut connection = pool.get().context("Failed to get database connection")?;

    match signup(
        &mut connection,
       user
    )
    .await
    {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "msg": "User created successfully",
            "success": true,
        }))),
        Err(e) => Err(IdentityError::UnexpectedError(e)),
        

    }
}