use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppStateData;

pub fn create_routes() -> OpenApiRouter<AppStateData> {
    OpenApiRouter::new().routes(routes!(health_check))
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Service Health", body = String)
    )
)]
pub async fn health_check() -> &'static str {
    "ok"
}
