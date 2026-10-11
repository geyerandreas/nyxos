use serde::Serialize;
use sqlx::Error;
use sqlx::SqlitePool;
use sqlx::prelude::*;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub password_hash: String,
}

pub async fn get(pool: SqlitePool, id: i32) -> Result<User, Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
}
