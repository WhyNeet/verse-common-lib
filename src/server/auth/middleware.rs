use std::future::{ready, Ready};

use actix_web::{error::ErrorUnauthorized, http::header, web, FromRequest};
use serde_json::json;

use super::{
    jwt::JwtAuth,
    token::{access::AccessToken, refresh::RefreshToken},
};

impl FromRequest for AccessToken {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let jwt_auth = req
            .app_data::<web::Data<JwtAuth>>()
            .expect("failed to retreive JwtAuth from app_data");

        let token = req.cookie("at").map(|c| c.value().to_string()).or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
        });

        if token.is_none() {
            return ready(Err(ErrorUnauthorized(
                json!({ "error": "You are not logged in." }),
            )));
        }

        let token = token.unwrap();
        let claims = jwt_auth.decode_jwt(&token);

        if claims.is_err() {
            return ready(Err(ErrorUnauthorized(
                json!({ "error": "You are not logged in." }),
            )));
        }

        let claims = claims.unwrap();

        ready(Ok(claims))
    }
}

impl FromRequest for RefreshToken {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let jwt_auth = req
            .app_data::<web::Data<JwtAuth>>()
            .expect("failed to reteive JwtAuth from app_data");

        let token = req
            .headers()
            .get(header::AUTHORIZATION)
            .map(|val| val.to_str().unwrap().split_at(7).1.to_string());

        if token.is_none() {
            return ready(Err(ErrorUnauthorized(
                json!({ "error": "Refresh token is not provided." }),
            )));
        }

        let token = token.unwrap();
        let token_data = jwt_auth.decode_jwt(&token);

        if token_data.is_err() {
            return ready(Err(ErrorUnauthorized(
                json!({ "error": "Invalid refresh token provided." }),
            )));
        }

        let token_data = token_data.unwrap();

        ready(Ok(token_data))
    }
}
