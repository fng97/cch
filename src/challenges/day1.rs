use actix_web::get;
use actix_web::{Error, FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};
use std::num::ParseIntError;
use std::ops::BitXor;
use tracing::info;

struct Ids(Vec<i32>);

const MAX_IDS: usize = 20;

impl FromRequest for Ids {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    #[tracing::instrument(name = "Parsing IDs from path", skip(req),
        fields(
            request.path = %req.path()
        ))]
    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let path = req.match_info().query("ids");

        let ids = match path
            .split('/')
            .map(|num_str| num_str.parse::<i32>())
            .collect::<Result<Vec<i32>, ParseIntError>>()
        {
            Ok(ids) => ids,
            Err(e) => {
                info!("Failed to parse an ID from the path: {:?}", e);
                return ready(Err(actix_web::error::ErrorBadRequest("Bad Request")));
            }
        };

        if ids.len() > MAX_IDS {
            info!("Too many IDs in the path. Max is {}", MAX_IDS);
            return ready(Err(actix_web::error::ErrorBadRequest("Bad Request")));
        }

        ready(Ok(Ids(ids)))
    }
}

#[tracing::instrument(name = "Recallibrating IDs", skip(ids))]
#[get("/1/{ids:.*}")] // match all
async fn recallibrate_ids(ids: Ids) -> String {
    cube_xor_of_ids(ids.0).to_string()
}

fn cube_xor_of_ids(ids: Vec<i32>) -> i32 {
    ids.iter().fold(0, |acc, id| acc.bitxor(id)).pow(3)
}

// just checking I've understood the bitxor trait
#[test]
fn int_xor_works() {
    assert_eq!(12, 4.bitxor(8));
}
