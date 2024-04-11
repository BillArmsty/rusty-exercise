use crate::types::PooledConnection;
use crate::{ types::User, repository::register_user };



pub async fn signup(
    connection: &mut PooledConnection,
    user: User
) -> Result<(), anyhow::Error> {

    let _ = register_user(connection, &user).await;
    Ok(())
}
