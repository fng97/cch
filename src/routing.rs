use actix_web::web::{Data, ServiceConfig};
use std::collections::HashMap;
use std::sync::Mutex;

use crate::challenges::day_01;
use crate::challenges::day_04;
use crate::challenges::day_06;
use crate::challenges::day_07;
use crate::challenges::day_08;
use crate::challenges::day_11;
use crate::challenges::day_12;
use crate::challenges::warmup;

pub fn config(cfg: &mut ServiceConfig) {
    let day_12_app_state = Data::new(day_12::AppState {
        timestamps: Mutex::new(HashMap::new()),
    });

    cfg.app_data(Data::new(reqwest::Client::new()))
        .app_data(day_12_app_state)
        .service(warmup::root)
        .service(warmup::fake_error)
        .service(day_01::recallibrate_ids)
        .service(day_04::combine_reindeer_strengths)
        .service(day_04::summarise_winners)
        .service(day_06::count_elves_and_shelves)
        .service(day_07::decode_cookie_recipe)
        .service(day_07::calculate_max_bakes)
        .service(day_08::get_pokemon_weight)
        .service(day_11::baubles_asset)
        .service(day_12::save_packet_id_with_timestamp)
        .service(day_12::time_elapsed_since_packet_id_saved);
}
