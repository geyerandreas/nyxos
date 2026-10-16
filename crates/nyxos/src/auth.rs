use axum::{
    extract::{FromRef, FromRequestParts},
    http::StatusCode,
};
use nyxos_auth::jwt::JwtService;
use tower_cookies::Cookies;

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
        let token = Cookies::from_request_parts(parts, state)
            .await
            .ok()
            .and_then(|cookies| {
                cookies
                    .get("auth_token")
                    .map(|cookie| cookie.value().to_owned())
            })
            .or_else(|| {
                parts
                    .headers
                    .get("authorization")
                    .and_then(|value| value.to_str().ok())
                    .and_then(|authorization| authorization.strip_prefix("Bearer "))
                    .map(str::to_owned)
            })
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let jwt = JwtService::from_ref(state);
        let claims = jwt
            .validate_token(&token)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let user_id = claims
            .sub
            .parse::<i32>()
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(Self { user_id })
    }
}
