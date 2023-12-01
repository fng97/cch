use actix_web::web::ServiceConfig;

use crate::routes::fake_error;
use crate::routes::root;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(root).service(fake_error);
}
