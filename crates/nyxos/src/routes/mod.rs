use crate::openapi::ApiDoc;
use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::AppStateData;

mod health;

pub fn create_router(state: AppStateData) -> Router {
    let (router, api) = OpenApiRouter::<AppStateData>::with_openapi(ApiDoc::openapi())
        .routes(routes!(say_hello))
        .nest("/api/v1", health::create_routes())
        .split_for_parts();
    return router
        .with_state(state) // Serve Swagger UI at /api/docs with OpenAPI spec at /api/openapi.json
        .merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", api));
}

#[utoipa::path(get, path = "/", responses((status = 200, description = "say hello")))]
async fn say_hello() -> &'static str {
    return "Hello, World";
}
