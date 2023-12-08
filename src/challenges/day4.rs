use actix_web::{post, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct Reindeer {
    strength: u8,
}

#[post("/4/strength")]
async fn combined_reindeer_strengths(reindeers: web::Json<Vec<Reindeer>>) -> String {
    reindeers
        .0
        .iter()
        .fold(0, |acc, r| acc + r.strength)
        .to_string()
}
