use actix_web::{post, web, Either};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    elf: u32,
}

#[derive(Serialize)]
struct ResponseWithShelves {
    elf: u32,
    #[serde(rename(serialize = "elf on a shelf"))]
    elf_on_a_shelf: u32,
    #[serde(rename(serialize = "shelf with no elf on it"))]
    shelf_with_no_elf_on_it: u32,
}

#[post("/6")]
async fn count_elves_and_shelves(
    body: String,
) -> Either<web::Json<Response>, web::Json<ResponseWithShelves>> {
    let elf_count = count_elves(&body);

    if !body.contains("elf on a shelf") {
        Either::Left(web::Json(Response { elf: elf_count }))
    } else {
        let elves_on_shelves_count = count_elves_on_shelves(&body);

        Either::Right(web::Json(ResponseWithShelves {
            elf: elf_count,
            elf_on_a_shelf: elves_on_shelves_count,
            shelf_with_no_elf_on_it: count_shelves(&body) - elves_on_shelves_count,
        }))
    }
}

fn count_elves(text: &str) -> u32 {
    text.matches("elf").count() as u32
}

fn count_elves_on_shelves(text: &str) -> u32 {
    text.matches("elf on a shelf").count() as u32
}

fn count_shelves(text: &str) -> u32 {
    text.matches("shelf").count() as u32
}
