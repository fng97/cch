use actix_web::{post, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ReindeerStats {
    strength: u32,
}

#[post("/4/strength")]
async fn combine_reindeer_strengths(reindeers: web::Json<Vec<ReindeerStats>>) -> String {
    reindeers
        .iter()
        .fold(0, |acc, r| acc + r.strength)
        .to_string()
}

#[derive(Deserialize)]
struct ReindeerStatsFull {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(alias = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u32,
}

#[derive(Serialize)]
struct Winners {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

#[post("/4/contest")]
async fn summarise_winners(reindeers: web::Json<Vec<ReindeerStatsFull>>) -> web::Json<Winners> {
    let loser: ReindeerStatsFull = ReindeerStatsFull {
        name: "Loser".to_string(),
        strength: 0,
        speed: f32::MIN,
        height: u32::MIN,
        antler_width: 0,
        snow_magic_power: u32::MIN,
        favorite_food: "a humble pie".to_string(),
        candies_eaten_yesterday: u32::MIN,
    };

    let mut fastest = &loser;
    let mut tallest = &loser;
    let mut magician = &loser;
    let mut consumer = &loser;

    for reindeer in reindeers.iter() {
        if reindeer.speed > fastest.speed {
            fastest = reindeer;
        }
        if reindeer.height > tallest.height {
            tallest = reindeer;
        }
        if reindeer.snow_magic_power > magician.snow_magic_power {
            magician = reindeer;
        }
        if reindeer.candies_eaten_yesterday > consumer.candies_eaten_yesterday {
            consumer = reindeer;
        }
    }

    web::Json(Winners {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    })
}
