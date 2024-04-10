use crate::{ types::User, repository::register_user };


use diesel::prelude::*;
use diesel::{ self };

pub async fn signup(
    conn: &mut PgConnection,
    user: User
) -> Result<(), anyhow::Error> {

    let pool = conn;
    let _ = register_user(pool, &user).await;
    Ok(())
}
