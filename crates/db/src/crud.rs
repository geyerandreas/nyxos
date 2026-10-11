use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use nyxos_settings::setup::Setup;
use serde::Deserialize;
use sqlx::SqlitePool;
use utoipa::ToSchema;

use crate::users::User;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserPayload {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl From<Setup> for CreateUserPayload {
    fn from(value: Setup) -> Self {
        Self {
            email: value.admin_email,
            name: value.admin_name,
            password: value.admin_password,
        }
    }
}

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
