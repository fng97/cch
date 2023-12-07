use actix_web::get;
use actix_web::{Error, FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};
use std::num::ParseIntError;
use std::ops::BitXor;

struct PathParams {
    ids: Vec<i32>,
}

impl FromRequest for PathParams {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let path = req.match_info().query("ids");

        let ids = match path
            .split('/')
            .map(|num_str| num_str.parse::<i32>())
            .collect::<Result<Vec<i32>, ParseIntError>>()
        {
            Ok(ids) => ids,
            Err(_) => return ready(Err(actix_web::error::ErrorBadRequest("Bad Request"))),
        };

        if ids.len() > 20 {
            return ready(Err(actix_web::error::ErrorBadRequest("Bad Request")));
        }

        ready(Ok(PathParams { ids }))
    }
}

#[get("/1/{ids:.*}")] // match all
async fn cube_xor_of_path_params(params: PathParams) -> String {
    recallibrate(params.ids).to_string()
}

fn recallibrate(ids: Vec<i32>) -> i32 {
    ids.iter().fold(0, |acc, id| acc.bitxor(id)).pow(3)
}

// just checking I've understood the bitxor trait
#[test]
fn int_xor_works() {
    assert_eq!(12, 4.bitxor(8));
}
