use super::{
    error_data::Error,
    security_data::{Claims, HashAlgorithm, HashData, PasswordStrengthIssue},
    utils::{DIGITS_REGEX, LOWER_CASE_LETTER_REGEX, SYMBOLS_REGEX, UPPER_CASE_LETTER_REGEX},
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use std::env;

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

pub fn generate_auth_token(user_id: String) -> String {
    // tbd maybe use key.pem with EncodingKey::from_rsa_pem(include_bytes!("privkey.pem"))?
    let encoding_key = EncodingKey::from_secret(JWT_SECRET.as_bytes());

    // tbd convert to SystemTime
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = now
        .checked_add_signed(chrono::Duration::days(30))
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

pub fn verify_auth_token(auth_token: String) -> bool {
    // tbd get real decoding errors
    decode::<Claims>(
        &auth_token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| Error::JWTTokenDecodingError)
    .is_ok()
}

fn get_jwt_secret() -> String {
    dotenv().ok();
    env::var("JWT_SECRET").expect("DATABASE_URL must be set")
}

// Argon2 with default params (Argon2id v19)
fn get_argon_instance<'a>() -> Argon2<'a> {
    Argon2::default()
}
