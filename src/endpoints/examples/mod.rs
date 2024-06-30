pub mod validate_json;
mod path_extraction;
mod request_params;
mod headers;

use axum::{
    routing::{get, post},
    Router,
};
use axum::http::Method;
use tower_http::cors::{CorsLayer, Any};

pub fn router() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    Router::new()
        .route("/json", get(validate_json::get_json))
        .route("/validate_json", get(validate_json::validated_json))
        .route("/path/:some_id", get(path_extraction::path_extraction))
        .route("/request_params", get(request_params::request_params))
        .route("/get_headers", get(headers::headers))
        .layer(cors)
}