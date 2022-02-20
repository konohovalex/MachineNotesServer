use super::{
    account_dto::{CredentialsDto, SignUpDataDto},
    account_interaction,
    postgres_database_connection::PostgresDatabaseConnectionPool,
};
use actix_web::{
    guard::{Delete, Post},
    web::{resource, scope, Data, Json},
    HttpRequest, Responder, Scope,
};

pub const ACCOUNT_PATH: &str = "/v1/account";
pub const SIGN_UP_PATH: &str = "/signUp";
pub const SIGN_IN_PATH: &str = "/signIn";
pub const DELETE_ACCOUNT_PATH: &str = "";
pub const REFRESH_TOKEN_PATH: &str = "/refreshToken";

// tbd make all routes consts
pub fn account_v1_scope() -> Scope {
    let sign_up_service_factory = resource(SIGN_UP_PATH).guard(Post()).to(sign_up);
    let sign_in_service_factory = resource(SIGN_IN_PATH).guard(Post()).to(sign_in);
    let delete_account_service_factory = resource(DELETE_ACCOUNT_PATH)
        .guard(Delete())
        .to(delete_account);
    let refresh_token_service_factory =
        resource(REFRESH_TOKEN_PATH).guard(Post()).to(refresh_token);
    scope(ACCOUNT_PATH)
        .service(sign_up_service_factory)
        .service(sign_in_service_factory)
        .service(delete_account_service_factory)
        .service(refresh_token_service_factory)
}

async fn sign_up(
    request: HttpRequest,
    database_connection_pool: Data<PostgresDatabaseConnectionPool>,
    sign_up_data_dto: Json<SignUpDataDto>,
) -> impl Responder {
    let profile_data_dto = account_interaction::sign_up(
        request,
        database_connection_pool.into_inner(),
        sign_up_data_dto.into_inner(),
    )
    .await;

    Json(profile_data_dto)
}

async fn sign_in(
    request: HttpRequest,
    database_connection_pool: Data<PostgresDatabaseConnectionPool>,
    credentials_dto: Json<CredentialsDto>,
) -> impl Responder {
    let profile_data_dto = account_interaction::sign_in(
        request,
        database_connection_pool.into_inner(),
        credentials_dto.into_inner(),
    )
    .await;

    Json(profile_data_dto)
}

async fn delete_account(
    request: HttpRequest,
    database_connection_pool: Data<PostgresDatabaseConnectionPool>,
) -> impl Responder {
    let profile_dto =
        account_interaction::delete_account(request, database_connection_pool.into_inner()).await;

    Json(profile_dto)
}

async fn refresh_token(
    request: HttpRequest,
    database_connection_pool: Data<PostgresDatabaseConnectionPool>,
    refresh_token: Json<String>,
) -> impl Responder {
    let auth_token_dto = account_interaction::refresh_token(
        request,
        database_connection_pool.into_inner(),
        refresh_token.into_inner(),
    )
    .await;

    Json(auth_token_dto)
}
