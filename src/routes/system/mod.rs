mod info;

use axum::{Router, routing::get};
use info::info;

pub fn routing() -> Router {
    Router::new()
        .route("/", get(info))
}