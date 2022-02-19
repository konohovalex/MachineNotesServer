use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenv::dotenv;
use std::{env, sync::Arc};

pub type PostgresDatabaseConnectionPool = Pool<ConnectionManager<PgConnection>>;
pub type PostgresDatabasePooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_database_connection_pool() -> PostgresDatabaseConnectionPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection_manager = ConnectionManager::new(database_url);

    Pool::builder().build(connection_manager).unwrap()
}

pub fn establish_database_connection(
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
) -> PostgresDatabasePooledConnection {
    database_connection_pool.get().unwrap()
}
