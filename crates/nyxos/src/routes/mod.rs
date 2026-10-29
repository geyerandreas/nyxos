use crate::openapi::ApiDoc;
use axum::{
    Router,
    extract::{Path, State},
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use nyxos_common::normalize_name;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;
mod auth;
mod health;
mod registry;
mod user;

pub fn create_router(state: AppState) -> Router {
    let api_router: OpenApiRouter<AppState> = OpenApiRouter::with_openapi(ApiDoc::openapi())
        // .route("/", get(embedded_static_root_handler))
        .nest("/api/v1/health", health::create_routes())
        .nest("/api/v1/users", user::create_routes())
        .nest("/api/v1/auth", auth::create_routes())
        .nest("/api/v1/registry", registry::create_routes())
        .route("/packages/{project}/{filename}", get(download_package))
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin([
                    HeaderValue::from_static("http://localhost:3000"),
                    HeaderValue::from_static("http://127.0.0.1:3000"),
                ])
                .allow_methods([
                    axum::http::Method::POST,
                    axum::http::Method::GET,
                    axum::http::Method::PATCH,
                    axum::http::Method::DELETE,
                ])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ])
                .allow_credentials(true),
        );

    let (router, api): (Router<AppState>, _) = api_router.split_for_parts();

    let router = router
        // Serve Swagger UI at /api/docs with OpenAPI spec at /api/openapi.json
        .merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", api));

    router.with_state(state)
}

async fn download_package(
    Path((project, filename)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Response {
    if filename.is_empty() || filename.contains('/') {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let project = normalize_name(&project);
    let key = format!("{project}/{filename}");
    let data = match state.storage.get(&key).await {
        Ok(data) => data,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    let mut response = data.into_response();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    response
}
