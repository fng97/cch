use actix_web::{get, Error, FromRequest, HttpRequest};
use base64::{engine::general_purpose, Engine as _};
use futures_util::future::{ready, Ready};

struct CookieRecipe(String);

impl FromRequest for CookieRecipe {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let recipe = match req.cookie("recipe") {
            Some(encoded) => encoded,
            None => return ready(Err(actix_web::error::ErrorBadRequest("Missing Cookie!"))),
        };

        let recipe = match general_purpose::STANDARD.decode(recipe.value()) {
            Ok(decoded) => decoded,
            Err(_) => return ready(Err(actix_web::error::ErrorBadRequest("Bad Cookie!"))),
        };

        match String::from_utf8(recipe) {
            Ok(recipe) => ready(Ok(CookieRecipe(recipe))),
            Err(_) => ready(Err(actix_web::error::ErrorBadRequest("Bad Cookie!"))),
        }
    }
}

#[get("/7/decode")]
async fn decode_cookie_recipe(recipe: CookieRecipe) -> String {
    recipe.0
}
