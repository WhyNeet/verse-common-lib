use actix_web::{get, HttpResponse};
use common::server::auth::token::access::AccessToken;

#[get("/api/jwt-auth-test")]
pub async fn jwt_auth_test(user: AccessToken) -> HttpResponse {
    HttpResponse::Ok().json(user)
}
