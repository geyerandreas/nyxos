use crate::error::Result;
use argon2::{Argon2, PasswordVerifier};
use argon2::{PasswordHash, PasswordHasher};

pub fn hash_password(password: &str) -> Result<String> {
    let result = Argon2::default().hash_password(password.as_bytes())?;
    Ok(result.to_string())
}

pub fn verify_password(password: &str, encoded_hash: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(encoded_hash) else {
        return false;
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
