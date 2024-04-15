use actix_session::Session;
use actix_web::{ web, HttpResponse };
use anyhow::Context;
use serde_json::json;
// use diesel::sql_types::Uuid;
// use uuid::Uuid;

use crate::{
    // messages::FetchAllUsers,
    services::get_all_users,
    types::{ response::IdentityError, PgPool },
};

use super::login::check_auth;


#[tracing::instrument(
    name = "Getting list of all users",
    skip_all,
    fields(user_id = tracing::field::Empty)
)]
pub async fn get_users(
    pool: web::Data<PgPool>,
    session: Session
) -> Result<HttpResponse, IdentityError> {
    let _ = check_auth(&session)?;
    // let _user_id = match user_id_result {
    //     Ok(user_id) => user_id,
    //     Err(e) => {
    //         return Err(IdentityError::InvalidCredentials(e));
    //     }
    // };

    let mut connection = pool.get().context("Failed to get database connection")?;

    match get_all_users(&mut connection).await {
        Ok(users) =>
            Ok(
                HttpResponse::Ok().json(
                    json!({
                    "msg": "Users fetched successfully",
                    "success": true,
                    "users": users
                })
                )
            ),
        Err(e) => Err(IdentityError::UnexpectedError(e)),
    }
}
