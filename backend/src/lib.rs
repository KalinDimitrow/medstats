use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceResponse},
    web, App,
};
use diesel::{pg::PgConnection, prelude::*, r2d2, r2d2::ConnectionManager};

use anyhow::Error;
use std::sync::Arc;
pub mod errors;
pub mod models;
pub mod schema;

pub mod endpoints;

type DB = Arc<r2d2::Pool<ConnectionManager<PgConnection>>>;

pub async fn configure_database() -> Result<DB, Error> {
    //  podman run --name data_base -e POSTGRES_USER=user -e POSTGRES_PASSWORD=password -e POSTGRES_DB=database -p 5432:5432 -d postgres
    let database_url = "postgres://user:password@localhost:5432/database"; //postgres://postgres:postgres@localhost/postgres";

    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let db = r2d2::Pool::builder()
        .max_size(2)
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");

    Ok(Arc::new(db))
}

pub fn create_app(
    pool: DB,
) -> App<
    impl ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(pool))
        .configure(endpoints::scoped_config)
}
