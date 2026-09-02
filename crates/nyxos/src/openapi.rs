use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(
    title = "nyxos API",
    version = "0.1.0",
    description = "Self-hosted Python registry"
))]
pub struct ApiDoc;
