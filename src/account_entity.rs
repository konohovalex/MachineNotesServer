use super::{
    schema::{auth_token, user_account},
    security::generate_password_hash,
    security_data::HashAlgorithm,
};
use diesel::{Insertable, Queryable};
use std::{
    fmt::{Display, Formatter},
    time::SystemTime,
};
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "user_account"]
pub struct InsertableUserAccountEntity {
    pub user_id: String,
    pub user_name: Option<String>,
    pub password_hash: Option<String>,
    pub password_hash_salt: Option<String>,
    pub password_hash_algorithm: Option<HashAlgorithm>,
}

impl InsertableUserAccountEntity {
    pub fn guest() -> Self {
        let uuid = Uuid::new_v4();

        InsertableUserAccountEntity {
            user_id: uuid.to_string(),
            user_name: None,
            password_hash: None,
            password_hash_salt: None,
            password_hash_algorithm: None,
        }
    }

    pub fn registered_user(user_name: String, password: String) -> Self {
        let uuid = Uuid::new_v4();

        let password_hash_data = generate_password_hash(password.as_bytes());

        InsertableUserAccountEntity {
            user_id: uuid.to_string(),
            user_name: Some(user_name),
            password_hash: Some(password_hash_data.hash),
            password_hash_salt: Some(password_hash_data.salt),
            password_hash_algorithm: Some(password_hash_data.algorithm),
        }
    }
}

#[derive(Queryable)]
pub struct UserAccountEntity {
    pub user_id: String,
    pub user_name: Option<String>,
    pub created_at: SystemTime,
    pub password_hash: Option<String>,
    pub password_hash_salt: Option<String>,
    pub password_hash_algorithm: Option<HashAlgorithm>,
}

impl Display for UserAccountEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(user_name) = self.user_name.as_ref() {
            write!(f, "{}", user_name)
        } else {
            write!(f, "guest")
        }
    }
}

#[derive(Insertable, Queryable)]
#[table_name = "auth_token"]
pub struct AuthTokenEntity {
    pub user_id: String,
    pub token: String,
}
