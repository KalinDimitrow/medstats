use axum::{
    async_trait,
    extract::Json,
    body::{HttpBody, Body},
    response::IntoResponse,
    http::{StatusCode},
    extract::{FromRequest, FromRequestParts, Request},
    RequestExt
};
use axum::extract::Path;
use serde_json::json;

pub async fn path_extraction(Path(id): Path<String>) -> impl IntoResponse {
    let var = json!({
        "Some_id": id
    });
    axum::Json(var)
}