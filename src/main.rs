use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

use cch::routing::config;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    Ok(config.into())
}
