use actix_web::{get, post, web, HttpResponse};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

pub struct AppState {
    pub timestamps: Mutex<HashMap<String, Instant>>,
}

#[post("/12/save/{packet_id}")]
async fn save_packet_id_with_timestamp(
    timestamps: web::Data<AppState>,
    packet_id: web::Path<String>,
) -> HttpResponse {
    let packet_id = packet_id.into_inner();
    let mut timestamps = timestamps
        .timestamps
        .lock()
        .expect("Failed to lock mutex for timestamps");

    timestamps.insert(packet_id, Instant::now());

    HttpResponse::Ok().finish()
}

#[get("/12/load/{packet_id}")]
async fn time_elapsed_since_packet_id_saved(
    timestamps: web::Data<AppState>,
    packet_id: web::Path<String>,
) -> String {
    let packet_id = packet_id.into_inner();
    let timestamps = timestamps
        .timestamps
        .lock()
        .expect("Failed to lock mutex for timestamps");

    timestamps
        .get(&packet_id)
        .expect("Failed to get timestamp for packet_id")
        .elapsed()
        .as_secs()
        .to_string()
}
