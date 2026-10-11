use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use serde::Deserialize;
use sqlx::SqlitePool;

use crate::users::User;

#[derive(Debug, Deserialize)]
pub struct UserPayload {
    pub name: String,
    pub email: String,
}

pub async fn update_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
    Json(payload): Json<UserPayload>,
) -> Result<Json<User>, StatusCode> {
    sqlx::query_as::<_, User>("UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING *")
        .bind(payload.name)
        .bind(payload.email)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
