use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use nyxos_appstate::AppState;
use nyxos_db::crud::{CreateUserPayload, User};
use nyxos_db::operations;
use sqlx::SqlitePool;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

/// Creates the user management routes
pub fn create_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        // User CRUD
        .routes(routes!(list_users, add, delete))
    // .routes(routes!(user::delete))
    // // User attributes
    // .routes(routes!(user::reset_pwd))
    // .routes(routes!(user::admin))
    // .routes(routes!(user::read_only))
    // // Current user (self-service)
    // .routes(routes!(user::change_pwd))
    // .routes(routes!(user::list_tokens, user::add_token))
    // .routes(routes!(user::delete_token))
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
async fn list_users(State(pool): State<SqlitePool>) -> Result<Json<Vec<User>>, StatusCode> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Create a new user
#[utoipa::path(
    post,
    path = "/",
    tag = "users",
    request_body = CreateUserPayload,
    responses(
        (status = 201, description = "User created successfully"),
        (status = 500, description = "Internal server error")
    ),
)]
async fn add(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    operations::create_user(&pool, &payload)
        .await
        .map(|user| (StatusCode::CREATED, Json(user)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Delete a user
#[utoipa::path(
    delete,
    path = "/{name}",
    tag = "users",
    params(
        ("name" = String, Path, description = "Username to delete")
    ),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User does not exist"),
        (status = 500, description = "Internal server error")
    ),
)]
pub async fn delete(
    State(pool): State<SqlitePool>,
    Path(name): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM users where name = $1")
        .bind(name)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
