use crate::error::Result;
use nyxos_auth::password::hash_password;
use nyxos_settings::setup::Setup;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Error;
use sqlx::SqlitePool;
use sqlx::prelude::*;
use utoipa::ToSchema;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub is_admin: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserCreate {
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserUpdate {
    pub name: Option<String>,
    pub email: Option<String>,
    pub is_admin: Option<bool>,
}

impl From<Setup> for UserCreate {
    fn from(value: Setup) -> Self {
        Self {
            email: value.admin_email,
            name: value.admin_name,
            password: value.admin_password,
            is_admin: true,
        }
    }
}

pub async fn get(pool: &SqlitePool, id: i32) -> Result<User, Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn update(pool: &SqlitePool, id: i32, user: &UserUpdate) -> Result<User, Error> {
    sqlx::query_as::<_, User>(
        "
        UPDATE users
        SET
            name = COALESCE($1, name),
            email = COALESCE($2, email)
            is_admin = COALESCE($3, is_admin)
        WHERE id = $3
        RETURNING *",
    )
    .bind(&user.name)
    .bind(&user.email)
    .bind(&user.is_admin)
    .bind(&id)
    .fetch_one(pool)
    .await
}

pub async fn create(pool: &SqlitePool, user: &UserCreate) -> Result<User> {
    let password_hash = hash_password(&user.password)?;
    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, password_hash, is_admin)
        VALUES ($1, $2, $3, $4)
        RETURNING *",
    )
    .bind(&user.name)
    .bind(&user.email)
    .bind(&password_hash)
    .bind(&user.is_admin)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn no_user_exists(pool: &SqlitePool) -> Result<bool> {
    // SQLite returns 1 (true) if a row exists, or 0 (false) if the table is empty.
    // We negate it with NOT to directly answer "is it empty?".
    let is_empty: bool = sqlx::query_scalar("SELECT NOT EXISTS (SELECT 1 FROM users)")
        .fetch_one(pool)
        .await?;

    Ok(is_empty)
}
