use actix_web::{
    App,
    web,
    dev::{ServiceFactory, ServiceResponse},
    body::MessageBody
};
use diesel::{pg::PgConnection, r2d2, r2d2::ConnectionManager, prelude::*};

use std::{sync::Arc};
use anyhow::Error;

pub mod endpoints;

type DB = Arc<r2d2::Pool<ConnectionManager<PgConnection>>>;

pub async fn configure_database() -> Result<DB, Error> {
    let database_url = "postgres://postgres:postgres@localhost/postgres";

    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let db = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");
    
    Ok(Arc::new(db))
}

pub fn create_app(pool: DB) -> App<
    impl ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new().app_data(web::Data::new(pool))
        .configure(endpoints::scoped_config)
}