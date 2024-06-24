use axum::{
    response::IntoResponse,
    http::{StatusCode},
    http::header::HeaderMap,
};


pub async fn headers(headers: HeaderMap) -> impl IntoResponse{
    (StatusCode::OK, format!("{:?}", headers))
}