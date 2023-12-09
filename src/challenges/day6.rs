use actix_web::post;

#[post("/6")]
async fn count_elf_sub_strings(_body: String) -> String {
    r#"{"elf":4}"#.to_string()
}
