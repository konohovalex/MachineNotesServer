use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CredentialsDto {
    #[serde(rename(deserialize = "userName"))]
    pub user_name: String,
    #[serde(rename(deserialize = "password"))]
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignUpDataDto {
    #[serde(rename(deserialize = "credentials"))]
    pub credentials_dto: Option<CredentialsDto>,
}

#[derive(Serialize)]
pub struct ProfileDto {
    #[serde(rename(serialize = "userId"))]
    pub user_id: String,
    #[serde(rename(serialize = "userName"))]
    pub user_name: Option<String>,
    #[serde(rename(serialize = "authToken"))]
    pub auth_token_dto: AuthTokenDto,
}

#[derive(Serialize)]
pub struct AuthTokenDto {
    #[serde(rename(serialize = "accessToken"))]
    pub access_token: String,
    #[serde(rename(serialize = "refreshToken"))]
    pub refresh_token: String,
}
