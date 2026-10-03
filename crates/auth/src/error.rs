use thiserror::Error;

pub type Result<T, E = AuthError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Could not hash the password")]
    HashPasswordError(#[from] argon2::password_hash::Error),
}
