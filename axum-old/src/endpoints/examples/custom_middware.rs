use axum::{
    extract::Request,
    http::{StatusCode, HeaderMap},
    middleware::{Next},
    response::{Response, IntoResponse},
    Extension,

};

use std::string::String;

pub async fn custom_header(headers: HeaderMap, mut request: Request, next: Next) -> Result<Response,StatusCode> {
    let message = headers.get("message").ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message.to_str().map_err(|_| StatusCode::BAD_REQUEST)?;
    let _ = request.extensions_mut().insert(CustomHeader{ message: message.to_owned()});
    let response = next.run(request).await;
    Ok(response)
}
#[derive(Clone)]
pub struct CustomHeader {
    pub message : String
}

pub async fn read_custom_header(Extension(header): Extension<CustomHeader>) -> impl IntoResponse {
    header.message
}