use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use nyxos_appstate::AppState;
use nyxos_db::users::{User, UserUpdate};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

/// Creates the user management routes
pub fn create_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        // User CRUD
        .routes(routes!(list_users, add))
        .routes(routes!(delete, get, update))
}

#[derive(Serialize, ToSchema)]
struct UserPublic {
    email: String,
    name: String,
}

impl From<User> for UserPublic {
    fn from(value: User) -> Self {
        Self {
            email: value.email,
            name: value.name,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserCreate {
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

impl From<UserCreate> for nyxos_db::users::UserCreate {
    fn from(value: UserCreate) -> Self {
        Self {
            name: value.name,
            email: value.email,
            password: value.password,
            is_admin: value.is_admin,
        }
    }
}

/// List all users
#[utoipa::path(
    get,
    path = "/",
    tag = "users",
    responses(
        (status = 200, description = "List of all users"),
        (status = 500, description = "Internal server error"),
    ),
)]
async fn list_users(State(pool): State<SqlitePool>) -> Result<Json<Vec<UserPublic>>, StatusCode> {
    let users = nyxos_db::users::get_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users.into_iter().map(UserPublic::from).collect()))
}

/// Create a new user
#[utoipa::path(
    post,
    path = "/",
    tag = "users",
    request_body = UserCreate,
    responses(
        (status = 201, description = "User created successfully"),
        (status = 500, description = "Internal server error")
    ),
)]
async fn add(
    State(pool): State<SqlitePool>,
    Json(payload): Json<UserCreate>,
) -> Result<(StatusCode, Json<UserPublic>), StatusCode> {
    let user = nyxos_db::users::UserCreate::from(payload);

    let x = nyxos_db::users::create(&pool, &user)
        .await
        // .map(|user| (StatusCode::CREATED, Json(user)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(UserPublic::from(x))))
}

/// Delete a user
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User Id to delete")
    ),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User does not exist"),
        (status = 500, description = "Internal server error")
    ),
)]
pub async fn delete(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM users where id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

/// Get a user
#[utoipa::path(
    get,
    path = "/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User Id")
    ),
    responses(
        (status = 200, description = "Found user"),
        (status = 404, description = "User does not exist"),
    ),
)]
pub async fn get(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<UserPublic>, StatusCode> {
    let user = nyxos_db::users::get(&pool, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(UserPublic::from(user)))
}

/// Update a user
#[utoipa::path(
    patch,
    path = "/{id}",
    tag = "users",
    request_body = UserUpdate,
    params(
        ("id" = i32, Path, description = "User Id")
    ),
    responses(
        (status = 500, description = "Internal server error")
    ),
)]
pub async fn update(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
    Json(payload): Json<UserUpdate>,
) -> Result<Json<UserPublic>, StatusCode> {
    let user = nyxos_db::users::update(&pool, id, &payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UserPublic::from(user)))
}
