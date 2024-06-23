pub mod validate_json;
mod path_extraction;

use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/json", get(crate::endpoints::examples::validate_json::get_json))
        .route("/validate_json", get(crate::endpoints::examples::validate_json::validated_json))
        .route("/path/:some_id", get(crate::endpoints::examples::path_extraction::path_extraction))
}