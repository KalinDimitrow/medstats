use actix_web::web;
use create_user::create_user;
mod create_user;
mod db_query;
mod hellow_world;

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hellow_world::hello)
        .service(hellow_world::echo)
        .service(web::resource("/create_user").route(web::get().to(create_user::create_user)))
        .service(web::resource("/find_user").route(web::get().to(create_user::test_user)))
        .service(web::resource("/manual_hello").route(web::get().to(hellow_world::manual_hello)))
        .service(web::resource("/db").route(web::get().to(db_query::simple_query)));
}
