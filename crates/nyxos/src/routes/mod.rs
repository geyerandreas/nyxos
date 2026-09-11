use crate::auth::AuthUser;
use crate::openapi::ApiDoc;
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::AppStateData;
mod auth;
mod health;

pub fn create_router(state: AppStateData) -> Router {
    let (router, api) = OpenApiRouter::<AppStateData>::with_openapi(ApiDoc::openapi())
        .routes(routes!(say_hello))
        .nest("/api/v1", health::create_routes())
        .split_for_parts();
    router
        .route(
            "/api/v1/users",
            get(nyxos_db::crud::list_users).post(nyxos_db::crud::create_user),
        )
        .route(
            "/api/v1/users/{id}",
            get(nyxos_db::crud::get_user)
                .put(nyxos_db::crud::update_user)
                .delete(nyxos_db::crud::delete_user),
        )
        .route("/api/v1/protected", get(protected_endpoint))
        .route("/api/v1/auth/login", axum::routing::post(auth::login))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
        .merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", api))
}

#[utoipa::path(get, path = "/", responses((status = 200, description = "say hello")))]
async fn say_hello() -> &'static str {
    return "Hello, World";
}

async fn protected_endpoint(AuthUser { user_id }: AuthUser) -> String {
    format!("Authenticated user: {user_id}")
}
