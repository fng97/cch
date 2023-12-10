use actix_web::{get, HttpResponse, Responder};

#[tracing::instrument]
#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[tracing::instrument(name = "Responding with fake")]
#[get("/-1/error")]
async fn fake_error() -> impl Responder {
    HttpResponse::InternalServerError().finish()
}
