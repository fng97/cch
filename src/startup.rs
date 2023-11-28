use actix_web::web::ServiceConfig;

use crate::routes::health_check;
use crate::routes::root;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(root).service(health_check);
}
