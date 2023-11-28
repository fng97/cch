use actix_web::{get, HttpResponse, Responder};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}
