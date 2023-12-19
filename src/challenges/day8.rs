use actix_web::{get, web};
use serde::Deserialize;

// TODO: Add error handling
// TODO: Mock requests in tests using wiremock. See
// https://www.lpalmieri.com/posts/how-to-write-a-rest-client-in-rust-with-reqwest-and-wiremock/#how-to-test-a-rest-client

#[derive(Deserialize)]
struct Pokemon {
    #[serde(rename(deserialize = "weight"))]
    weight_hectograms: f32,
}

impl Pokemon {
    fn weight_kg(&self) -> f32 {
        self.weight_hectograms / 10.
    }
}

#[get("/8/weight/{pokedex}")]
async fn get_pokemon_weight(path: web::Path<u32>, client: web::Data<reqwest::Client>) -> String {
    let pokedex = path.into_inner();

    let response = client
        .get(format!("https://pokeapi.co/api/v2/pokemon/{pokedex}"))
        .send()
        .await
        .expect("Failed to execute request");

    let contents = response
        .text()
        .await
        .expect("Failed to get response contents");

    let pokemon: Pokemon = serde_json::from_str(&contents).unwrap();

    pokemon.weight_kg().to_string()
}
