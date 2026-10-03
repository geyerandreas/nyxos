use crate::{
    crud::{CreateUserPayload, User},
    error::Result,
};
use nyxos_auth::password::hash_password;
use nyxos_settings::setup::Setup;
use sqlx::{SqlitePool, query_scalar};

pub async fn no_user_exists(pool: &SqlitePool) -> Result<bool> {
    // SQLite returns 1 (true) if a row exists, or 0 (false) if the table is empty.
    // We negate it with NOT to directly answer "is it empty?".
    let is_empty: bool = query_scalar("SELECT NOT EXISTS (SELECT 1 FROM users)")
        .fetch_one(pool)
        .await?;

    Ok(is_empty)
}

pub async fn create_user(pool: &SqlitePool, payload: &CreateUserPayload) -> Result<User> {
    let password_hash = hash_password(&payload.password)?;
    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, name, email",
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(password_hash)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub trait Credentials {
    fn password(&self) -> String;
    fn email(&self) -> String;
    fn username(&self) -> String;
}

pub async fn add_initial_admin_account(pool: &SqlitePool, user: Setup) -> Result<User> {
    create_user(pool, &CreateUserPayload::from(user)).await
}
