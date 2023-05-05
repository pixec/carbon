mod info;

use axum::{routing::get, Router};
use info::info;

pub fn routing() -> Router {
    Router::new().route("/", get(info))
}
