use actix_web::{http::header::ContentType, test, App};

#[actix_web::test]
async fn healthcheck_works() {
    let app = test::init_service(App::new().service(common::server::routes::healthcheck)).await;
    let req = test::TestRequest::default()
        .uri("/api/healthcheck")
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
