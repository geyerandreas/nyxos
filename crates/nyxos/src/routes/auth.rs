use axum::{Json, extract::State, http::StatusCode};
use nyxos_auth::{jwt::JwtService, password::verify_password};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, prelude::FromRow};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: &'static str,
}

#[derive(Debug, FromRow)]
struct LoginUser {
    id: i32,
    password_hash: String,
}

pub async fn login(
    State(pool): State<SqlitePool>,
    State(jwt): State<JwtService>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user =
        sqlx::query_as::<_, LoginUser>("SELECT id, password_hash FROM users WHERE email = $1")
            .bind(payload.email)
            .fetch_optional(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

    if !verify_password(&payload.password, &user.password_hash) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let access_token = jwt
        .create_token(user.id, 60 * 60)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        access_token,
        token_type: "Bearer",
    }))
}
