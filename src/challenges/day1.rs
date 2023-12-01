use std::ops::BitXor;

use actix_web::{get, web, Result};

#[get("/1/{x}/{y}")]
async fn cube_xor_of_path_params(path: web::Path<(u32, u32)>) -> Result<String> {
    let (x, y) = path.into_inner();
    let xored = x.bitxor(y);
    let cubed = xored.pow(3);

    Ok(cubed.to_string())
}

// just checking I've understood the bitxor trait
#[test]
fn int_xor_works() {
    assert_eq!(12, 4.bitxor(8));
}
