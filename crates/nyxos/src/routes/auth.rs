use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use cookie::time::Duration;
use cookie::{Cookie, SameSite};
use nyxos_appstate::AppState;
use nyxos_auth::{jwt::JwtService, password::verify_password};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, prelude::FromRow};
use tower_cookies::Cookies;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

const AUTH_COOKIE_NAME: &str = "auth_token";
const AUTH_COOKIE_MAX_AGE: Duration = Duration::hours(24);

/// Creates the auth routes
pub fn create_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(login))
        .routes(routes!(logout))
        .routes(routes!(status))
        .routes(routes!(register))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: &'static str,
}

#[derive(Debug, FromRow)]
struct LoginUser {
    id: i32,
    password_hash: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub password: String,
}

/// Login
#[utoipa::path(
    post,
    path = "/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Authorized", body = LoginResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
)]
pub async fn login(
    cookies: Cookies,
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
        .create_token(user.id, AUTH_COOKIE_MAX_AGE.whole_seconds())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    cookies.add(auth_cookie(&access_token, AUTH_COOKIE_MAX_AGE));

    Ok(Json(LoginResponse {
        access_token,
        token_type: "Bearer",
    }))
}

/// Logout
#[utoipa::path(
    post,
    path = "/logout",
    tag = "auth",
    responses(
        (status = 200, description = "Logged out"),
    ),
)]
pub async fn logout(cookies: Cookies) -> impl IntoResponse {
    cookies.add(auth_cookie("", Duration::ZERO));

    StatusCode::OK
}

/// Returns whether the current session is authenticated.
#[utoipa::path(
    get,
    path = "/status",
    tag = "auth",
    responses(
        (status = 200, description = "Authenticated"),
        (status = 401, description = "Unauthorized"),
    ),
)]
pub async fn status(cookies: Cookies, State(jwt): State<JwtService>) -> StatusCode {
    let Some(cookie) = cookies.get(AUTH_COOKIE_NAME) else {
        return StatusCode::UNAUTHORIZED;
    };

    if jwt.validate_token(cookie.value()).is_ok() {
        StatusCode::OK
    } else {
        StatusCode::UNAUTHORIZED
    }
}

/// Register
#[utoipa::path(
    post,
    path = "/register",
    tag = "auth",
    request_body = Register,
    responses(
        (status = 201, description = "User created successfully"),
        (status = 500, description = "Internal server error")
    ),
)]
async fn register(
    State(pool): State<SqlitePool>,
    Json(payload): Json<Register>,
) -> Result<(StatusCode, Json<nyxos_db::users::User>), StatusCode> {
    nyxos_db::users::create(
        &pool,
        &nyxos_db::users::UserCreate {
            is_admin: false,
            name: payload.name,
            email: payload.email,
            password: payload.password,
        },
    )
    .await
    .map(|user| (StatusCode::CREATED, Json(user)))
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn auth_cookie(value: &str, max_age: Duration) -> Cookie<'static> {
    Cookie::build((AUTH_COOKIE_NAME, value.to_owned()))
        .path("/")
        .max_age(max_age)
        .http_only(true)
        .secure(cookie_secure())
        .same_site(SameSite::Lax)
        .build()
}

fn cookie_secure() -> bool {
    std::env::var("NYXOS_COOKIE_SECURE").is_ok_and(|value| value.eq_ignore_ascii_case("true"))
}
