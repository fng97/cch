use actix_web::web::ServiceConfig;

use crate::challenges::day1;
use crate::challenges::day4;
use crate::challenges::warmup;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(warmup::root)
        .service(warmup::fake_error)
        .service(day1::cube_xor_of_path_params)
        .service(day4::combined_reindeer_strengths);
}
