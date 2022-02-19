use super::{
    account_entity::{AuthTokenEntity, InsertableUserAccountEntity, UserAccountEntity},
    postgres_database_connection::{establish_database_connection, PostgresDatabaseConnectionPool},
    schema::{auth_token, user_account},
};
use diesel::prelude::*;
use std::sync::Arc;

pub async fn insert_user_account(
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    insertable_user_account_entity: InsertableUserAccountEntity,
) -> UserAccountEntity {
    let database_connection = establish_database_connection(database_connection_pool);

    let user_account_entity = diesel::insert_into(user_account::table)
        .values(&insertable_user_account_entity)
        .get_result(&database_connection)
        .expect("Error inserting user account");

    println!("Sucessfully inserted {}'s account", user_account_entity);

    user_account_entity
}

pub async fn get_user_account_by_user_name(
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    user_name: String,
) -> UserAccountEntity {
    let database_connection = establish_database_connection(database_connection_pool);

    let user_account_entity = user_account::table
        .filter(user_account::user_name.eq(Some(user_name)))
        .first(&database_connection)
        .expect("Error loading user account");

    println!("Sucessfully loaded {}'s account", user_account_entity);

    user_account_entity
}

pub async fn delete_user_account(
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    auth_token: String,
) -> usize {
    let database_connection = establish_database_connection(database_connection_pool.clone());

    let auth_token_entity = get_auth_token(database_connection_pool, auth_token).await;

    let delete_source =
        user_account::table.filter(user_account::user_id.eq(auth_token_entity.user_id));

    let num_deleted = diesel::delete(delete_source)
        .execute(&database_connection)
        .expect("Error deleting user account");

    println!("Deleted {} users", num_deleted);

    num_deleted
}

pub async fn insert_auth_token(
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    auth_token_entity: AuthTokenEntity,
) -> AuthTokenEntity {
    let database_connection = establish_database_connection(database_connection_pool);

    let auth_token_entity = diesel::insert_into(auth_token::table)
        .values(&auth_token_entity)
        .get_result(&database_connection)
        .expect("Error inserting auth token");

    auth_token_entity
}

pub async fn get_auth_token(
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    auth_token: String,
) -> AuthTokenEntity {
    let database_connection = establish_database_connection(database_connection_pool);

    let auth_token_entity = auth_token::table
        .filter(auth_token::token.eq(auth_token))
        .first(&database_connection)
        .expect("Error loading auth token");

    auth_token_entity
}

pub async fn get_auth_token_by_user_id(
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    user_id: String,
) -> AuthTokenEntity {
    let database_connection = establish_database_connection(database_connection_pool);

    let auth_token_entity = auth_token::table
        .filter(auth_token::user_id.eq(user_id))
        .first(&database_connection)
        .expect("Error loading auth token");

    auth_token_entity
}

pub async fn delete_auth_token(
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    auth_token: String,
) -> usize {
    let database_connection = establish_database_connection(database_connection_pool);

    let num_deleted = diesel::delete(auth_token::table.filter(auth_token::token.eq(auth_token)))
        .execute(&database_connection)
        .expect("Error deleting auth token");

    println!("Deleted {} users", num_deleted);

    num_deleted
}
