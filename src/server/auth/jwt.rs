use std::{env, sync::Arc};

use actix_web::cookie::Cookie;
use chrono::Duration;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;

use super::token::{access::AccessToken, refresh::RefreshToken};

#[derive(Clone)]
pub struct JwtAuth {
    jwt_secret: Arc<str>,
    jwt_rt_max_age: Duration,
    jwt_at_max_age: Duration,
}

impl JwtAuth {
    pub fn new(jwt_secret: Arc<str>, jwt_rt_max_age: Duration, jwt_at_max_age: Duration) -> Self {
        Self {
            jwt_secret,
            jwt_rt_max_age,
            jwt_at_max_age,
        }
    }

    pub fn new_from_env() -> anyhow::Result<Self> {
        let jwt_secret = env::var("JWT_SECRET")?.into();

        let jwt_rt_max_age = env::var("JWT_RT_MAX_AGE")?.parse::<i64>()?;
        let jwt_rt_max_age = Duration::seconds(jwt_rt_max_age);

        let jwt_at_max_age = env::var("JWT_AT_MAX_AGE")?.parse::<i64>()?;
        let jwt_at_max_age = Duration::seconds(jwt_at_max_age);

        Ok(Self {
            jwt_secret,
            jwt_rt_max_age,
            jwt_at_max_age,
        })
    }
}

impl JwtAuth {
    pub fn encode_jwt<T: serde::Serialize>(&self, claims: T) -> anyhow::Result<String> {
        Ok(jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?)
    }

    pub fn decode_jwt<T: DeserializeOwned>(&self, token: &str) -> anyhow::Result<T> {
        Ok(jsonwebtoken::decode(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )?
        .claims)
    }
}

impl JwtAuth {
    fn generate_rt(&self, uuid: String) -> anyhow::Result<String> {
        let rt = RefreshToken::new(uuid, self.jwt_rt_max_age);
        let rt = self.encode_jwt(rt)?;

        Ok(rt)
    }

    pub fn generate_at(&self, uuid: String) -> anyhow::Result<String> {
        let at = AccessToken::new(uuid, self.jwt_at_max_age);
        let at = self.encode_jwt(at)?;

        Ok(at)
    }

    /// Returns (access_token, refresh_token)
    pub fn generate_tokens(&self, uuid: String) -> anyhow::Result<(String, String)> {
        let at = self.generate_at(uuid.clone())?;
        let rt = self.generate_rt(uuid)?;

        Ok((at, rt))
    }
}

impl JwtAuth {
    pub fn generate_cookie(at: &str) -> Cookie {
        Cookie::build("at", at)
            .path("/")
            .same_site(actix_web::cookie::SameSite::Lax)
            .http_only(true)
            .finish()
    }
}
