use actix_web::{get, post};

#[post("/12/save/{packet_id}")]
async fn save_packet_id_with_timestamp() -> String {
    "".to_string()
}

#[get("/12/load/{packet_id}")]
async fn time_elapsed_since_packet_id_saved() -> String {
    "1".to_string()
}
