pub mod validate_json;
mod path_extraction;
mod request_params;
mod headers;
mod shared_data;
mod custom_middware;

use axum::{
    routing::{get},
    Router,
    Extension,
    http::Method,
    middleware::from_fn
};
use tower_http::cors::{CorsLayer, Any};
#[derive(Clone)]
struct SharedData {
    message: String
}

pub fn router() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData{ message: "some message".to_owned() };
    Router::new()
        .route("/json", get(validate_json::get_json))
        .route("/validate_json", get(validate_json::validated_json))
        .route("/path/:some_id", get(path_extraction::path_extraction))
        .route("/request_params", get(request_params::request_params))
        .route("/get_headers", get(headers::headers))
        .route("/shared_data", get(shared_data::shared_data))
        .layer(Extension(shared_data))
        .layer(cors)
        .merge(Router::new()
            .route("/custom_header", get(custom_middware::read_custom_header))
            .layer(from_fn(custom_middware::custom_header)))
}