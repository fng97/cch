use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/-1/error")]
async fn fake_error() -> impl Responder {
    HttpResponse::InternalServerError()
}
