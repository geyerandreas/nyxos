use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

#[derive(Clone)]
pub struct JwtService {
    secret: Box<[u8]>,
}

impl JwtService {
    pub fn new(secret: impl AsRef<[u8]>) -> Self {
        Self {
            secret: secret.as_ref().into(),
        }
    }

    pub fn create_token(&self, user_id: i32, ttl_seconds: i64) -> Result<String, Error> {
        let expiration = Utc::now().timestamp() + ttl_seconds;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, Error> {
        let token_data = decode(
            token,
            &DecodingKey::from_secret(&self.secret),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &[u8] = b"a-development-secret-that-is-long-enough";

    #[test]
    fn creates_and_validates_token() {
        let service = JwtService::new(SECRET);
        let token = service
            .create_token(42, 60)
            .expect("token should be created");
        let claims = service
            .validate_token(&token)
            .expect("token should be valid");
        assert_eq!(claims.sub, "42")
    }

    #[test]
    fn rejects_token_with_wrong_secret() {
        let service = JwtService::new(SECRET);
        let other = JwtService::new(b"a-different-secret-that-is-long-enough");
        let token = service
            .create_token(42, 60)
            .expect("token should be created");
        assert!(other.validate_token(&token).is_err());
    }

    #[test]
    fn rejects_expired_token() {
        let service = JwtService::new(SECRET);
        let token = service
            .create_token(42, -120) // allow for clock-skew leeway
            .expect("token should be created");
        assert!(service.validate_token(&token).is_err())
    }

    #[test]
    fn rejects_invalid_token() {
        let service = JwtService::new(SECRET);

        assert!(service.validate_token("not-a-jwt").is_err());
    }
}
