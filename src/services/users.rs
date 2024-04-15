use crate::types::{ PooledConnection, User };
use crate::repository::fetch_all_users;

pub async fn get_all_users(
    connection: &mut PooledConnection
    // user_id: Uuid
) -> Result<Vec<User>, anyhow::Error> {
    let users = fetch_all_users(connection).await.map_err(|e| {
        tracing::error!("Failed to fetch users: {:?}", e);
        anyhow::anyhow!("Failed to fetch users")
    })?;

    Ok(users)
}
