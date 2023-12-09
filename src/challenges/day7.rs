use actix_web::get;

#[get("/7/decode")]
async fn decode_recipe() -> String {
    r#"{"flour":100,"chocolate chips":20}"#.to_string()
}
