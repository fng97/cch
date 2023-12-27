use actix_web::web::{Data, ServiceConfig};

use crate::challenges::day_01;
use crate::challenges::day_04;
use crate::challenges::day_06;
use crate::challenges::day_07;
use crate::challenges::day_08;
use crate::challenges::day_11;
use crate::challenges::warmup;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.app_data(Data::new(reqwest::Client::new()))
        .service(warmup::root)
        .service(warmup::fake_error)
        .service(day_01::recallibrate_ids)
        .service(day_04::combine_reindeer_strengths)
        .service(day_04::summarise_winners)
        .service(day_06::count_elves_and_shelves)
        .service(day_07::decode_cookie_recipe)
        .service(day_07::calculate_max_bakes)
        .service(day_08::get_pokemon_weight)
        .service(day_11::baubles_asset);
}
