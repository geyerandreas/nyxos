use axum::{
    extract::{FromRef, FromRequestParts},
    http::StatusCode,
};
use nyxos_auth::jwt::JwtService;

pub struct AuthUser {
    pub user_id: i32,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    JwtService: FromRef<S>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let authorization = parts
            .headers
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = authorization
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let jwt = JwtService::from_ref(state);
        let claims = jwt
            .validate_token(token)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let user_id = claims
            .sub
            .parse::<i32>()
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(Self { user_id })
    }
}
