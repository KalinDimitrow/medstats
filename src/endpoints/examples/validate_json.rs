
// use async_trait::async_trait;

use axum::{
    async_trait,
    extract::Json,
    body::{HttpBody, Body},
    response::IntoResponse,
    http::{StatusCode},
    extract::{FromRequest, FromRequestParts, Request},
    RequestExt
};

use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};
// Example of how to use Json extractor to parse the body of request
// and return json as a body of response
pub async fn get_json(Json(payload): Json<TestJason>) -> impl IntoResponse {

    (StatusCode::OK, Json(payload)).into_response()
}

pub async fn validated_json(user: ExtractAndValidateUser) -> impl IntoResponse {
    (StatusCode::OK, Json(user)).into_response()
}

#[derive(Serialize, Deserialize,Clone)]
pub struct TestJason {
    data: String
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ExtractAndValidateUser {
    #[validate(email)]
    user : String,
    #[validate(length(min = 8), custom(function = "custom_validation_func"))]
    password : String,
    summary : Option<String>
}

fn custom_validation_func(password: &str) -> Result<(), ValidationError> {
    let (has_uppercase, has_lowercase, has_special_symbol) = password.chars().fold((false, false, false), |(has_uppercase, has_lowercase, has_special), c| {
        (
            has_uppercase || c.is_uppercase(),
            has_lowercase || c.is_lowercase(),
            has_special || !c.is_alphanumeric(),
        )
    });

    if !has_uppercase { return Err(ValidationError::new("Username has to have at least one uppercase letter")) }
    if !has_lowercase { return Err(ValidationError::new("Username has to have at least one lowercase letter")) }
    if !has_special_symbol { return Err(ValidationError::new("Username has to have at least one special symbol")) }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = custom_validation_func("G0sh@");
        assert_eq!(result, Ok(()));
    }
}


#[async_trait]
impl FromRequest<(), Body> for ExtractAndValidateUser
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<Body>, _ : &()) -> Result<Self, Self::Rejection> {
        let Json(user) = req.extract::<Json<ExtractAndValidateUser>, _>().await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;

        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }

        Ok(user)
    }
}



