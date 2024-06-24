use axum::{
    body::{HttpBody},
    response::IntoResponse,
    http::{StatusCode},
    RequestExt
};
use axum::extract::{Query};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ValidatedParams {
    #[validate(length(min = 5))]
    user: std::string::String,
    #[validate(range(min = 18, max = 20))]
    age: u32,
}
pub async fn request_params(Query(params) : Query<ValidatedParams>) -> impl IntoResponse {
    match params.validate() {
        Ok(params) => {
            (StatusCode::OK,  format!(""))
        }
        Err(errors) => {
            (StatusCode::BAD_REQUEST, format!("{}", errors))
        }
    }
}