use actix_web::web::ServiceConfig;

use crate::challenges::warmup;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(warmup::root).service(warmup::fake_error);
}
