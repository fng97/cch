use actix_web::{get, web, Error, FromRequest, HttpRequest};
use base64::{engine::general_purpose, Engine as _};
use futures_util::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

struct CookieRecipe(String);

impl FromRequest for CookieRecipe {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let recipe = match req.cookie("recipe") {
            Some(encoded) => encoded,
            None => return ready(Err(actix_web::error::ErrorBadRequest("Missing Cookie!"))),
        };

        match base_64_decode(recipe.value()) {
            Ok(recipe) => ready(Ok(CookieRecipe(recipe))),
            Err(error) => ready(Err(error)),
        }
    }
}

fn base_64_decode(encoded: &str) -> Result<String, Error> {
    let error = actix_web::error::ErrorBadRequest("Bad Cookie!");

    let decoded = match general_purpose::STANDARD.decode(encoded.as_bytes()) {
        Ok(decoded) => decoded,
        Err(_) => return Err(error),
    };

    match String::from_utf8(decoded) {
        Ok(decoded) => Ok(decoded),
        Err(_) => Err(error),
    }
}

#[get("/7/decode")]
async fn decode_cookie_recipe(recipe: CookieRecipe) -> String {
    recipe.0
}

#[derive(Deserialize, Serialize)]
struct Ingredients {
    flour: u32,
    sugar: u32,
    butter: u32,
    #[serde(rename = "baking powder")]
    baking_powder: u32,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: u32,
}

#[derive(Deserialize, Serialize)]
struct RecipeAndPantry {
    recipe: Ingredients,
    pantry: Ingredients,
}

#[derive(Serialize)]
struct MaxBakesAndLeftovers {
    cookies: u32,
    pantry: Ingredients,
}

impl FromRequest for RecipeAndPantry {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let recipe_and_pantry = match req.cookie("recipe") {
            Some(encoded) => encoded,
            None => return ready(Err(actix_web::error::ErrorBadRequest("Missing Cookie!"))),
        };

        let recipe_and_pantry = match base_64_decode(recipe_and_pantry.value()) {
            Ok(recipe_and_pantry) => recipe_and_pantry,
            Err(error) => return ready(Err(error)),
        };

        let recipe_and_pantry: RecipeAndPantry = match from_str(&recipe_and_pantry) {
            Ok(recipe_and_pantry) => recipe_and_pantry,
            Err(_) => return ready(Err(actix_web::error::ErrorBadRequest("Bad Cookie!"))),
        };

        ready(Ok(recipe_and_pantry))
    }
}

#[get("/7/bake")]
async fn calculate_max_bakes(
    recipe_and_pantry: RecipeAndPantry,
) -> web::Json<MaxBakesAndLeftovers> {
    let (recipe, pantry) = (&recipe_and_pantry.recipe, &recipe_and_pantry.pantry);
    let max_bakes = max_bakes_for_recipe(recipe, pantry);

    let leftovers = Ingredients {
        flour: pantry.flour - (recipe.flour * max_bakes),
        sugar: pantry.sugar - (recipe.sugar * max_bakes),
        butter: pantry.butter - (recipe.butter * max_bakes),
        baking_powder: pantry.baking_powder - (recipe.baking_powder * max_bakes),
        chocolate_chips: pantry.chocolate_chips - (recipe.chocolate_chips * max_bakes),
    };

    web::Json(MaxBakesAndLeftovers {
        cookies: max_bakes,
        pantry: leftovers,
    })
}

fn max_bakes_for_recipe(recipe: &Ingredients, pantry: &Ingredients) -> u32 {
    [
        max_bakes_for_ingredient(recipe.flour, pantry.flour),
        max_bakes_for_ingredient(recipe.sugar, pantry.sugar),
        max_bakes_for_ingredient(recipe.butter, pantry.butter),
        max_bakes_for_ingredient(recipe.baking_powder, pantry.baking_powder),
        max_bakes_for_ingredient(recipe.chocolate_chips, pantry.chocolate_chips),
    ]
    .into_iter()
    .min()
    .unwrap_or(0)
}

fn max_bakes_for_ingredient(required: u32, available: u32) -> u32 {
    available / required
}
