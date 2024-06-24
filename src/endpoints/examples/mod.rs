pub mod validate_json;
mod path_extraction;
mod request_params;

use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/json", get(validate_json::get_json))
        .route("/validate_json", get(validate_json::validated_json))
        .route("/path/:some_id", get(path_extraction::path_extraction))
        .route("/request_params", get(request_params::request_params))
}