use actix_web::web::ServiceConfig;

use crate::challenges::day1;
use crate::challenges::day4;
use crate::challenges::warmup;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(warmup::root)
        .service(warmup::fake_error)
        .service(day1::recallibrate_ids)
        .service(day4::combine_reindeer_strengths)
        .service(day4::summarise_winners);
}
