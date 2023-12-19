use actix_web::{get, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pokemon {
    #[serde(rename(deserialize = "weight"))]
    weight_hectograms: u32,
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

    let weight_kg: f32 = pokemon.weight_hectograms as f32 / 10.;

    weight_kg.to_string()
}
