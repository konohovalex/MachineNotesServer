use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize)]
pub struct CredentialsDto {
    #[serde(rename(deserialize = "userName"))]
    pub user_name: String,
    #[serde(rename(deserialize = "password"))]
    pub password: String,
}

impl Display for CredentialsDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.user_name)
    }
}

#[derive(Deserialize)]
pub struct AuthDataDto {
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
    pub auth_token: String,
}
