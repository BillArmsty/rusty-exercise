use diesel::{r2d2::ConnectionManager, PgConnection};

pub type  PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;