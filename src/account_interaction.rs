use crate::account_database::get_auth_token_by_user_id;

use super::{
    account_database::{
        self, delete_auth_token, delete_user_account, get_auth_token,
        get_user_account_by_user_name, insert_user_account,
    },
    account_dto::{AuthDataDto, CredentialsDto, ProfileDto},
    account_entity::{AuthTokenEntity, InsertableUserAccountEntity, UserAccountEntity},
    postgres_database_connection::PostgresDatabaseConnectionPool,
    security::{check_password_strength, generate_auth_token, verify_auth_token, verify_password},
    security_data::PasswordStrengthIssue,
};
use actix_web::HttpRequest;
use std::sync::Arc;

const AUTH_TOKEN_HEADER_KEY: &str = "authToken";
const AUTH_TOKEN_BEARER_PREFIX: &str = "Bearer ";

pub async fn register_user_account(
    request: HttpRequest,
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    credentials_dto: CredentialsDto,
) -> ProfileDto {
    println!("Received {}", credentials_dto);

    match check_password_strength(&credentials_dto.password) {
        PasswordStrengthIssue::None => {
            let insertable_user_account_entity = InsertableUserAccountEntity::registered_user(
                credentials_dto.user_name,
                credentials_dto.password,
            );

            // tbd insert user and token in one transaction
            let user_account_entity = insert_user_account(
                database_connection_pool.clone(),
                insertable_user_account_entity,
            )
            .await;

            let auth_token_entity = get_auth_token_for_user(
                database_connection_pool.clone(),
                user_account_entity.user_id.clone(),
                true,
            )
            .await;

            user_sucessfully_authorized(
                request,
                database_connection_pool,
                user_account_entity,
                auth_token_entity,
            )
            .await
        }
        _ => {
            // tbd
            panic!("Password is weak")
        }
    }
}

pub async fn authorize_user(
    request: HttpRequest,
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    auth_data_dto: AuthDataDto,
) -> ProfileDto {
    match auth_data_dto.credentials_dto {
        Some(credentials) => {
            authorize_as_user(request, database_connection_pool, credentials).await
        }
        None => authorize_as_guest(database_connection_pool).await,
    }
}

pub async fn refresh_auth_token(
    request: HttpRequest,
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    old_refresh_token: String,
) -> String {
    let old_auth_token = get_auth_token_from_request(request);

    let old_auth_token_entity =
        get_auth_token(database_connection_pool.clone(), old_auth_token.clone()).await;

    delete_auth_token(database_connection_pool.clone(), old_auth_token).await;

    let user_id = old_auth_token_entity.user_id;

    // tbd deletion and addition must be in one transaction
    let auth_token_entity = insert_auth_token(user_id, database_connection_pool.clone()).await;

    auth_token_entity.token
}

pub async fn delete_user(
    request: HttpRequest,
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
) -> ProfileDto {
    let auth_token = get_auth_token_from_request(request);

    delete_user_account(database_connection_pool.clone(), auth_token).await;

    authorize_as_guest(database_connection_pool).await
}

async fn get_auth_token_for_user(
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    user_id: String,
    newly_registered: bool,
) -> AuthTokenEntity {
    // tbd check, that user's token is alive, if not - refresh it
    if newly_registered {
        insert_auth_token(user_id, database_connection_pool.clone()).await
    } else {
        get_auth_token_by_user_id(database_connection_pool, user_id).await
    }
}

async fn user_sucessfully_authorized(
    request: HttpRequest,
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    user_account_entity: UserAccountEntity,
    auth_token_entity: AuthTokenEntity,
) -> ProfileDto {
    // We need to delete user's guest account to avoid zombie user accounts
    let guest_auth_token = get_auth_token_from_request(request);
    delete_user_account(database_connection_pool.clone(), guest_auth_token).await;

    form_profile_dto(user_account_entity, auth_token_entity).await
}

fn get_auth_token_from_request(request: HttpRequest) -> String {
    // tbd error handling
    let auth_token = request
        .headers()
        .get(AUTH_TOKEN_HEADER_KEY)
        .unwrap()
        .to_str()
        .unwrap();

    if auth_token.starts_with(AUTH_TOKEN_BEARER_PREFIX) {
        let auth_token = auth_token
            .trim_start_matches(AUTH_TOKEN_BEARER_PREFIX)
            .to_owned();

        if verify_auth_token(auth_token.clone()) {
            auth_token
        } else {
            panic!("Auth token was not verified")
        }
    } else {
        panic!("Auth token must start with \"Bearer \" prefix")
    }
}

async fn insert_auth_token(
    user_id: String,
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
) -> AuthTokenEntity {
    let auth_token_entity = form_auth_token_entity(user_id).await;

    account_database::insert_auth_token(database_connection_pool, auth_token_entity).await
}

async fn form_auth_token_entity(user_id: String) -> AuthTokenEntity {
    let auth_token = generate_auth_token(user_id.clone());

    AuthTokenEntity {
        user_id: user_id,
        token: auth_token,
    }
}

async fn form_profile_dto(
    user_account_entity: UserAccountEntity,
    auth_token_entity: AuthTokenEntity,
) -> ProfileDto {
    ProfileDto {
        user_id: user_account_entity.user_id.to_string(),
        user_name: user_account_entity.user_name,
        auth_token: auth_token_entity.token,
    }
}

async fn authorize_as_user(
    request: HttpRequest,
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    credentials_dto: CredentialsDto,
) -> ProfileDto {
    println!("{}", credentials_dto);

    let user_account_entity =
        get_user_account_by_user_name(database_connection_pool.clone(), credentials_dto.user_name)
            .await;

    let password_hash = user_account_entity.password_hash.as_ref().unwrap().as_str();

    if verify_password(credentials_dto.password.as_bytes(), password_hash) {
        let auth_token_entity = get_auth_token_for_user(
            database_connection_pool.clone(),
            user_account_entity.user_id.clone(),
            false,
        )
        .await;

        user_sucessfully_authorized(
            request,
            database_connection_pool,
            user_account_entity,
            auth_token_entity,
        )
        .await
    } else {
        // tbd
        panic!("Password was not verified")
    }
}

async fn authorize_as_guest(
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
) -> ProfileDto {
    // tbd insert user and token in one transaction
    let insertable_user_account_entity = InsertableUserAccountEntity::guest();

    let user_account_entity = insert_user_account(
        database_connection_pool.clone(),
        insertable_user_account_entity,
    )
    .await;

    let auth_token_entity = insert_auth_token(
        user_account_entity.user_id.clone(),
        database_connection_pool,
    )
    .await;

    form_profile_dto(user_account_entity, auth_token_entity).await
}
