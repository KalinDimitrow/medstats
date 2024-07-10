use axum::Extension;
use axum::response::IntoResponse;
use crate::endpoints::examples::SharedData;

pub async fn shared_data(Extension(shared_data) : Extension<SharedData>) -> impl IntoResponse {
    shared_data.message
}