use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header::ACCEPT},
};

pub enum ResponseFormat {
    Html,
    Json,
}

impl<S> FromRequestParts<S> for ResponseFormat
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        if let Some(accept) = parts.headers.get(ACCEPT).and_then(|v| v.to_str().ok()) {
            if accept.contains("application/vnd.pypi.simple.v1+json") {
                return Ok(ResponseFormat::Json);
            }
        }
        Ok(ResponseFormat::Html)
    }
}
