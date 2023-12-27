use actix_files::NamedFile;
use actix_web::{get, Responder};

#[get("/11/assets/decoration.png")]
async fn baubles_asset() -> impl Responder {
    NamedFile::open_async("assets/decoration.png").await
}
