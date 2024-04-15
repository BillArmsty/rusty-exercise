use crate::types::PooledConnection;
use crate::{ types::User, repository::create_user };



pub async fn signup(
    connection: &mut PooledConnection,
    user: User
) -> Result<(), anyhow::Error> {

    let _ = create_user(connection, &user).await;
    Ok(())
}
