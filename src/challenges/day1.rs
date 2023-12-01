use std::ops::BitXor;

use actix_web::{get, web, Either, HttpResponse, Result};

// TODO: Find a better way of validating the input, regex or type?
#[get("/1/{params:.*}")] // match all path params
async fn cube_xor_of_path_params(path: web::Path<String>) -> Either<HttpResponse, Result<String>> {
    let params = path.into_inner();
    let params = params.split('/').collect::<Vec<&str>>();

    let mut result = 0;

    for number in params {
        match number.parse::<i128>() {
            Ok(n) => result = result.bitxor(n),
            Err(_) => return Either::Left(HttpResponse::BadRequest().body("Bad Request")),
        }
    }

    let cubed = result.pow(3);

    Either::Right(Ok(cubed.to_string()))
}

// just checking I've understood the bitxor trait
#[test]
fn int_xor_works() {
    assert_eq!(12, 4.bitxor(8));
}
