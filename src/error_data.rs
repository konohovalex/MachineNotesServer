use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("JWT token creation error")]
    JWTTokenCreationError,
    #[error("JWT token decoding error")]
    JWTTokenDecodingError,
}
