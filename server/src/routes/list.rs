use axum::{http::StatusCode, Json};
use serde::Serialize;

use crate::manager;

#[derive(Serialize)]
pub struct ListResponse {
    // List of USB ports
    devices: Vec<String>,
}

pub async fn list() -> (StatusCode, Json<ListResponse>) {
    let devices = manager::DEVICE_MANAGER
        .lock()
        .await
        .devices
        .iter()
        .map(|device| device.system_port.clone())
        .collect();

    (StatusCode::OK, Json(ListResponse { devices }))
}
