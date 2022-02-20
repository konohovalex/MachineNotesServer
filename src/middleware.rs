use super::{account_api, security};
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};

pub async fn bearer_auth_validator(
    service_request: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = service_request
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    let path = service_request.path();

    println!("{}:{}", service_request.method().as_str(), &path);

    if path.ends_with(account_api::SIGN_UP_PATH)
        || path.ends_with(account_api::SIGN_IN_PATH)
        || path.ends_with(account_api::REFRESH_TOKEN_PATH)
    {
        Ok(service_request)
    } else if security::verify_jwt(credentials.token()) {
        Ok(service_request)
    } else {
        Err(AuthenticationError::from(config).into())
    }
}
