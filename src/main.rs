use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

use fng_cch::run;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    run().await
}
