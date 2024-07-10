use actix_web::{
    App,
    dev::{ServiceFactory, ServiceResponse},
    body::MessageBody
};

pub mod endpoints;

pub fn create_app() -> App<
    impl ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new().configure(endpoints::scoped_config)
}