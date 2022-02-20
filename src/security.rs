use super::{
    error_data::Error,
    security_data::{AuthToken, Claims, HashAlgorithm, HashData, PasswordStrengthIssue},
    utils::{DIGITS_REGEX, LOWER_CASE_LETTER_REGEX, SYMBOLS_REGEX, UPPER_CASE_LETTER_REGEX},
};
use actix_web::http::HeaderMap;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use std::env;

const AUTHORIZATION_HEADER_KEY: &str = "Authorization";
const BEARER_PREFIX: &str = "Bearer ";

lazy_static! {
    static ref JWT_SECRET: String = get_jwt_secret();
}

// Requirements:
// 1. be 8 to 16 symbols long
// 2. contain at least one uppercase letter
// 3. contain at least one lowercase letter
// 4. contain at least one digit
// 5. contain at least one /p{Punct} symbol
// 6. have at least one unique symbol for each four
//
// We don't currently check, that password is in worst passwords list,
// because at 2022 there is no password in such lists, that meet such requirements
pub fn check_password_strength(password: &str) -> PasswordStrengthIssue {
    // tbd should this be here or on client side?
    // if password.len() < 8 {
    //     PasswordStrengthIssue::TooShort
    // } else if password.len() > 16 {
    //     PasswordStrengthIssue::TooLong
    // } else if password.contains(char::is_whitespace)
    // || password.as_bytes().iter().any(u8::is_ascii_whitespace) {
    // PasswordStrengthIssue::ContainsWhitespace }
    // else if !UPPER_CASE_LETTER_REGEX.is_match(password) {
    //     PasswordStrengthIssue::ContainsNoUpperCaseLetter
    // } else if !LOWER_CASE_LETTER_REGEX.is_match(password) {
    //     PasswordStrengthIssue::ContainsNoLowerCaseLetter
    // } else if !DIGITS_REGEX.is_match(password) {
    //     PasswordStrengthIssue::ContainsNoDigit
    // } else if !SYMBOLS_REGEX.is_match(password) {
    //     PasswordStrengthIssue::ContainsNoSymbol
    // } else {
    //     PasswordStrengthIssue::None
    // }
    PasswordStrengthIssue::None
}

pub fn generate_password_hash(password: &[u8]) -> HashData {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = get_argon_instance();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();

    HashData {
        salt: salt.as_str().to_string(),
        hash: password_hash,
        algorithm: HashAlgorithm::Argon2idV19,
    }
}

pub fn verify_password(password: &[u8], password_hash: &str) -> bool {
    // Verify password against PHC string.
    //
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    let parsed_hash = PasswordHash::new(password_hash).unwrap();

    Argon2::default()
        .verify_password(password, &parsed_hash)
        .is_ok()
}

pub fn generate_auth_token(user_id: String) -> AuthToken {
    let access_token = generate_access_token(user_id.clone());
    let refresh_token = generate_refresh_token(user_id);
    AuthToken {
        access_token: access_token,
        refresh_token: refresh_token,
    }
}

pub fn get_access_token_from_request_headers(headers: &HeaderMap) -> String {
    let untrimmed_access_token = get_untrimmed_access_token_from_request_headers(headers);
    trim_access_token(untrimmed_access_token).to_owned()
}

pub fn verify_access_token_from_request_headers(headers: &HeaderMap) -> bool {
    let untrimmed_access_token = get_untrimmed_access_token_from_request_headers(headers);
    if untrimmed_access_token.starts_with(BEARER_PREFIX) {
        let trimmed_access_token = trim_access_token(untrimmed_access_token);
        // tbd get real decoding errors
        verify_jwt(trimmed_access_token)
    } else {
        false
    }
}

pub fn verify_jwt(jwt: &str) -> bool {
    decode::<Claims>(jwt, &get_decoding_key(), &get_jwt_validation_algorithm())
        .map_err(|_| Error::JWTTokenDecodingError)
        .is_ok()
}

pub fn refresh_access_token(refresh_token: String, user_id: String) -> String {
    if verify_jwt(refresh_token.as_str()) {
        generate_access_token(user_id)
    } else {
        panic!("Refresh token was not verified")
    }
}

// Argon2 with default params (Argon2id v19)
fn get_argon_instance<'a>() -> Argon2<'a> {
    Argon2::default()
}

fn get_jwt_secret() -> String {
    dotenv().ok();
    env::var("JWT_SECRET").expect("DATABASE_URL must be set")
}

fn get_untrimmed_access_token_from_request_headers<'a>(headers: &'a HeaderMap) -> &'a str {
    headers
        .get(AUTHORIZATION_HEADER_KEY)
        .unwrap()
        .to_str()
        .unwrap()
}

fn trim_access_token(access_token: &str) -> &str {
    access_token.trim_start_matches(BEARER_PREFIX)
}

fn generate_access_token(user_id: String) -> String {
    // tbd
    generate_jwt(user_id, Duration::days(1))
}

fn generate_refresh_token(user_id: String) -> String {
    generate_jwt(user_id, Duration::days(30))
}

fn generate_jwt(user_id: String, valid_for: Duration) -> String {
    // tbd maybe use key.pem with EncodingKey::from_rsa_pem(include_bytes!("privkey.pem"))?
    let encoding_key = EncodingKey::from_secret(JWT_SECRET.as_bytes());

    let now = Utc::now();
    let iat = now.timestamp();
    let exp = now
        .checked_add_signed(valid_for)
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        iat: iat as usize,
        exp: exp as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &encoding_key)
        .map_err(|_| Error::JWTTokenCreationError)
        .unwrap()
}

fn get_decoding_key() -> DecodingKey {
    DecodingKey::from_secret(JWT_SECRET.as_bytes())
}

fn get_jwt_validation_algorithm() -> Validation {
    Validation::new(get_jwt_algorithm())
}

fn get_jwt_algorithm() -> Algorithm {
    Algorithm::HS512
}
