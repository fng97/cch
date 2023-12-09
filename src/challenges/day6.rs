use actix_web::{post, web};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    elf: u32,
}

#[post("/6")]
async fn count_elf_sub_strings(body: String) -> web::Json<Response> {
    web::Json(Response {
        elf: body.matches("elf").count() as u32,
    })
}
