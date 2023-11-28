use actix_web::web::ServiceConfig;

use crate::routes::health_check;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(health_check);
}
