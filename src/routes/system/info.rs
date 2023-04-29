use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Info {
    uptime: i64,
}

pub async fn info() -> (StatusCode, Json<Info>) {
    let info = Info {
        uptime: 0,
    };

    (StatusCode::OK, Json(info))
}