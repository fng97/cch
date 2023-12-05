use actix_web::get;
use actix_web::{Error, FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};
use std::num::ParseIntError;
use std::ops::BitXor;
use std::str::FromStr;

enum PathParamsError {
    ParseError(ParseIntError),
    TooManyIds,
}

impl From<ParseIntError> for PathParamsError {
    fn from(err: ParseIntError) -> Self {
        PathParamsError::ParseError(err)
    }
}

struct PathParams {
    ids: Vec<i32>,
}

impl FromStr for PathParams {
    type Err = PathParamsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ids: Vec<i32> = s
            .split('/')
            .map(|num_str| num_str.parse::<i32>())
            .collect::<Result<Vec<i32>, ParseIntError>>()?;

        // check number of ids is between 1 and 20
        if ids.len() > 20 {
            return Err(PathParamsError::TooManyIds);
        }

        Ok(PathParams { ids })
    }
}

impl FromRequest for PathParams {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let path = req.match_info().query("ids");

        match PathParams::from_str(path) {
            Ok(params) => ready(Ok(params)),
            Err(_) => ready(Err(actix_web::error::ErrorBadRequest("Bad Request"))),
        }
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
