use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(health_check);
}
