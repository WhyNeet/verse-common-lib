mod shared;

use actix_web::{http::header, test, web, App};
use chrono::Duration;
use common::server::auth::{jwt::JwtAuth, token::access::AccessToken};

use crate::shared::setup;

#[test]
pub async fn jwt_auth_works() {
    setup::setup_env();

    let claims = AccessToken::new("test_id".to_string(), Duration::seconds(1));
    let jwt_auth = JwtAuth::new_from_env().expect("failed to initialize JwtAuth from env");

    let token = jwt_auth.encode_jwt(claims);
    assert!(token.is_ok());

    let claims = jwt_auth.decode_jwt::<AccessToken>(&token.unwrap());
    assert!(claims.is_ok());
}

#[actix_web::test]
pub async fn jwt_middleware_works() {
    setup::setup_env();

    let jwt_auth = JwtAuth::new_from_env().expect("failed to initialize JwtAuth from env");
    let claims = AccessToken::new("test_id".to_string(), Duration::seconds(10));

    let token = jwt_auth
        .encode_jwt(&claims)
        .expect("failed to encode JwtUserClaims");
    let token = format!("Bearer {token}");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(jwt_auth))
            .service(shared::handlers::jwt_auth_test),
    )
    .await;
    let req = test::TestRequest::default()
        .uri("/api/jwt-auth-test")
        .insert_header((header::AUTHORIZATION, token))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success())
}
