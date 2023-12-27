use actix_web::{post, web};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    elf: u32,
    #[serde(rename(serialize = "elf on a shelf"))]
    elf_on_a_shelf: u32,
    #[serde(rename(serialize = "shelf with no elf on it"))]
    shelf_with_no_elf_on_it: u32,
}

#[post("/6")]
async fn count_elves_and_shelves(body: String) -> web::Json<Response> {
    let elf_on_a_shelf_count = count_substrs(&body, "elf on a shelf");

    web::Json(Response {
        elf: count_substrs(&body, "elf"),
        elf_on_a_shelf: elf_on_a_shelf_count,
        shelf_with_no_elf_on_it: count_substrs(&body, "shelf") - elf_on_a_shelf_count,
    })
}

/// Count the number of times the substring occurs in the text.
///
/// This caught me out! Must support overlapping substrings. E.g. "elf on a
/// shelf on a shelf" should match "elf on a shelf" twice.
/// `matches(substr).count()` would only match it once.
fn count_substrs(text: &str, substr: &str) -> u32 {
    text.chars()
        .collect::<Vec<_>>()
        .windows(substr.len()) // create sliding window of the size of the pattern
        .filter(|window| window.iter().collect::<String>() == substr)
        .count() as u32
}
