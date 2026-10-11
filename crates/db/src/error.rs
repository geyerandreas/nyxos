use nyxos_auth::error::AuthError;

pub type Result<T, E = DatabaseError> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    DataBaseError(#[from] sqlx::Error),

    #[error("password hashing failed: {0}")]
    PasswordHash(#[from] AuthError),
}
