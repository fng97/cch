use actix_web::{get, web, Either, HttpResponse};
use std::num::ParseIntError;
use std::ops::BitXor;

#[get("/1/{tail:.*}")] // match all
async fn cube_xor_of_path_params(path: web::Path<String>) -> Either<String, HttpResponse> {
    let ids = match extract_path_params(&path.into_inner()) {
        Ok(p) => p,
        Err(_) => return Either::Right(HttpResponse::BadRequest().body("Bad Request")),
    };

    // check number of ids is between 1 and 20
    if ids.len() > 20 {
        return Either::Right(HttpResponse::BadRequest().body("Bad Request"));
    }

    Either::Left(recallibrate(ids).to_string())
}

fn extract_path_params(path: &str) -> Result<Vec<i32>, ParseIntError> {
    let params = path.split('/').collect::<Vec<&str>>();
    let params = params.iter().map(|s| s.trim()).collect::<Vec<&str>>();

    params.iter().map(|s| s.parse::<i32>()).collect()
}

fn recallibrate(ids: Vec<i32>) -> i32 {
    ids.iter().fold(0, |acc, id| acc.bitxor(id)).pow(3)
}

// just checking I've understood the bitxor trait
#[test]
fn int_xor_works() {
    assert_eq!(12, 4.bitxor(8));
}
