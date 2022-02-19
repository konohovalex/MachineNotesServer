use super::{
    account_dto::{AuthDataDto, CredentialsDto},
    account_interaction::{authorize_user, delete_user, refresh_auth_token, register_user_account},
    postgres_database_connection::PostgresDatabaseConnectionPool,
};
use actix_web::{
    delete, post,
    web::{scope, Data, Json},
    HttpRequest, Responder, Scope,
};

pub fn account_v1_scope() -> Scope {
    scope("/v1/account")
        .service(sign_up)
        .service(sign_in)
        .service(refresh_token)
        .service(delete_account)
}

#[post("/signUp")]
async fn sign_up(
    request: HttpRequest,
    database_connection_pool: Data<PostgresDatabaseConnectionPool>,
    credentials_dto: Json<CredentialsDto>,
) -> impl Responder {
    let profile_data_dto = register_user_account(
        request,
        database_connection_pool.into_inner(),
        credentials_dto.into_inner(),
    )
    .await;

    Json(profile_data_dto)
}

#[post("/signIn")]
async fn sign_in(
    request: HttpRequest,
    database_connection_pool: Data<PostgresDatabaseConnectionPool>,
    auth_data_dto: Json<AuthDataDto>,
) -> impl Responder {
    let profile_data_dto = authorize_user(
        request,
        database_connection_pool.into_inner(),
        auth_data_dto.into_inner(),
    )
    .await;

    Json(profile_data_dto)
}

#[post("/refreshToken")]
async fn refresh_token(
    request: HttpRequest,
    database_connection_pool: Data<PostgresDatabaseConnectionPool>,
    old_refresh_token: Json<String>,
) -> impl Responder {
    let token_data_dto = refresh_auth_token(
        request,
        database_connection_pool.into_inner(),
        old_refresh_token.into_inner(),
    )
    .await;

    Json(token_data_dto)
}

#[delete("/delete")]
async fn delete_account(
    request: HttpRequest,
    database_connection_pool: Data<PostgresDatabaseConnectionPool>,
) -> impl Responder {
    // tbd get user_id from Header
    let profile_dto = delete_user(request, database_connection_pool.into_inner()).await;

    Json(profile_dto)
}
