use super::{
    account_database,
    account_dto::{AuthTokenDto, CredentialsDto, ProfileDto, SignUpDataDto},
    account_entity::{InsertableUserAccountEntity, UserAccountEntity},
    postgres_database_connection::PostgresDatabaseConnectionPool,
    security,
    security_data::PasswordStrengthIssue,
};
use actix_web::HttpRequest;
use std::sync::Arc;

pub async fn sign_up(
    request: HttpRequest,
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    sign_up_data_dto: SignUpDataDto,
) -> ProfileDto {
    match sign_up_data_dto.credentials_dto {
        Some(credentials_dto) => {
            sign_up_as_user(request, database_connection_pool, credentials_dto).await
        }
        None => sign_up_as_guest(database_connection_pool).await,
    }
}

pub async fn sign_in(
    request: HttpRequest,
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    credentials_dto: CredentialsDto,
) -> ProfileDto {
    println!("Received {}", &credentials_dto.user_name);

    let user_account_entity = account_database::get_user_account_by_user_name(
        database_connection_pool.clone(),
        credentials_dto.user_name,
    )
    .await;

    let password_hash = user_account_entity.password_hash.as_ref().unwrap().as_str();

    if security::verify_password(credentials_dto.password.as_bytes(), password_hash) {
        user_sucessfully_authorized(request, database_connection_pool, user_account_entity).await
    } else {
        panic!("Password was not verified")
    }
}

pub async fn delete_account(
    request: HttpRequest,
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
) -> ProfileDto {
    let access_token = security::get_access_token_from_request_headers(request.headers());
    account_database::delete_user_account(database_connection_pool.clone(), access_token).await;

    sign_up_as_guest(database_connection_pool).await
}

pub async fn refresh_token(
    request: HttpRequest,
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    refresh_token: String,
) -> AuthTokenDto {
    println!("Refresh token");

    let access_token = security::get_access_token_from_request_headers(request.headers());

    println!("{}", &access_token);

    let user_account_entity = account_database::get_user_account_by_access_token(
        database_connection_pool.clone(),
        access_token,
    )
    .await;

    if refresh_token == user_account_entity.refresh_token {
        // tbd if refresh token was expired, then create completely new AuthToken
        let new_access_token = security::refresh_access_token(
            refresh_token.clone(),
            user_account_entity.user_id.clone(),
        );

        let updated_user_account_entity = account_database::update_auth_token(
            database_connection_pool,
            user_account_entity.user_id,
            new_access_token,
            user_account_entity.refresh_token,
        )
        .await;

        AuthTokenDto {
            access_token: updated_user_account_entity.access_token,
            refresh_token: updated_user_account_entity.refresh_token,
        }
    } else {
        panic!("Wrong refresh token")
    }
}

async fn sign_up_as_user(
    request: HttpRequest,
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    credentials_dto: CredentialsDto,
) -> ProfileDto {
    println!("Received {}", &credentials_dto.user_name);

    match security::check_password_strength(&credentials_dto.password) {
        PasswordStrengthIssue::None => {
            let insertable_user_account_entity = InsertableUserAccountEntity::registered_user(
                credentials_dto.user_name,
                credentials_dto.password,
            );

            let user_account_entity = account_database::insert_user_account(
                database_connection_pool.clone(),
                insertable_user_account_entity,
            )
            .await;

            user_sucessfully_authorized(request, database_connection_pool, user_account_entity)
                .await
        }
        _ => {
            panic!("Password is weak")
        }
    }
}

async fn user_sucessfully_authorized(
    request: HttpRequest,
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
    user_account_entity: UserAccountEntity,
) -> ProfileDto {
    // We need to delete user's guest account to avoid zombie user accounts
    let guest_access_token = security::get_access_token_from_request_headers(request.headers());
    account_database::delete_user_account(database_connection_pool.clone(), guest_access_token)
        .await;

    map_user_account_entity_to_profile_dto(user_account_entity).await
}

async fn sign_up_as_guest(
    database_connection_pool: Arc<PostgresDatabaseConnectionPool>,
) -> ProfileDto {
    let insertable_user_account_entity = InsertableUserAccountEntity::guest();

    let user_account_entity = account_database::insert_user_account(
        database_connection_pool.clone(),
        insertable_user_account_entity,
    )
    .await;

    map_user_account_entity_to_profile_dto(user_account_entity).await
}

async fn map_user_account_entity_to_profile_dto(
    user_account_entity: UserAccountEntity,
) -> ProfileDto {
    let auth_token_dto = AuthTokenDto {
        access_token: user_account_entity.access_token,
        refresh_token: user_account_entity.refresh_token,
    };

    ProfileDto {
        user_id: user_account_entity.user_id.to_string(),
        user_name: user_account_entity.user_name,
        auth_token_dto: auth_token_dto,
    }
}
