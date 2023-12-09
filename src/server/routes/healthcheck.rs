use actix_web::HttpResponse;

#[actix_web::get("/api/healthcheck")]
pub async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().finish()
}
