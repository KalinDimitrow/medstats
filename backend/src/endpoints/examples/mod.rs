use actix_web::{web};
mod hellow_world;
mod db_query;

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(hellow_world::hello)
        .service(hellow_world::echo)
        .service(
            web::resource("/manual_hello")
                .route(web::get().to(hellow_world::manual_hello))
        )
        .service(
            web::resource("/db")
                .route(web::get().to(db_query::simple_query))
        );
}