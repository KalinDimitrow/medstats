use actix_web::web;

pub mod examples;

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/examples").configure(examples::scoped_config));
}