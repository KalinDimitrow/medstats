pub mod validate_json;

use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/json", get(crate::endpoints::examples::validate_json::get_json))
        .route("/validate_json", get(crate::endpoints::examples::validate_json::validated_json))
}