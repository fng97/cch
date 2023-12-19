use actix_web::web::{Data, ServiceConfig};

use crate::challenges::day1;
use crate::challenges::day4;
use crate::challenges::day6;
use crate::challenges::day7;
use crate::challenges::day8;
use crate::challenges::warmup;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.app_data(Data::new(reqwest::Client::new()))
        .service(warmup::root)
        .service(warmup::fake_error)
        .service(day1::recallibrate_ids)
        .service(day4::combine_reindeer_strengths)
        .service(day4::summarise_winners)
        .service(day6::count_elves_and_shelves)
        .service(day7::decode_cookie_recipe)
        .service(day7::calculate_max_bakes)
        .service(day8::get_pokemon_weight);
}
