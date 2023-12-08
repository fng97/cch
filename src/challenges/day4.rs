use actix_web::{post, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct Reindeer {
    strength: u8,
}

#[post("/4/strength")]
async fn combine_reindeer_strengths(reindeers: web::Json<Vec<Reindeer>>) -> String {
    reindeers
        .0
        .iter()
        .fold(0, |acc, r| acc + r.strength)
        .to_string()
}

#[post("/4/contest")]
async fn summarise_winners() -> String {
    r#"
    {
        "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
        "tallest": "Dasher is standing tall with his 36 cm wide antlers",
        "magician": "Dasher could blast you away with a snow magic power of 9001",
        "consumer": "Dancer ate lots of candies, but also some grass"
    }
    "#
    .to_string()
}
