use super::{
    account_entity::{InsertableUserAccountEntity, UserAccountEntity},
    postgres_database_connection::{establish_database_connection, PostgresDatabaseConnectionPool},
    schema::user_account,
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
        .expect("Error loading user account with user name");

    println!("Sucessfully loaded {}'s account", user_account_entity);

    user_account_entity
}

pub async fn get_user_account_by_access_token(
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    access_token: String,
) -> UserAccountEntity {
    let database_connection = establish_database_connection(database_connection_pool);

    let user_account_entity = user_account::table
        .filter(user_account::access_token.eq(access_token))
        .first(&database_connection)
        .expect("Error loading user account with access token");

    println!("Sucessfully loaded {}'s account", user_account_entity);

    user_account_entity
}

pub async fn update_auth_token(
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    user_id: String,
    access_token: String,
    refresh_token: String,
) -> UserAccountEntity {
    let database_connection = establish_database_connection(database_connection_pool);

    let source = user_account::table.find(user_id);
    let user_account_entity = diesel::update(source)
        .set((
            (user_account::access_token.eq(access_token)),
            (user_account::refresh_token.eq(refresh_token)),
        ))
        .get_result(&database_connection)
        .expect("Error loading user account with access token");

    println!("Sucessfully updated {}'s account", user_account_entity);

    user_account_entity
}

pub async fn delete_user_account(
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    access_token: String,
) -> usize {
    let database_connection = establish_database_connection(database_connection_pool.clone());

    let delete_source = user_account::table.filter(user_account::access_token.eq(access_token));
    let num_deleted = diesel::delete(delete_source)
        .execute(&database_connection)
        .expect("Error deleting user account");

    println!("Deleted {} users", num_deleted);

    num_deleted
}
