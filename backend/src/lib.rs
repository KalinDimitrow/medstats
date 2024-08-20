use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    body::MessageBody,
    cookie::{time, Key},
    dev::{ServiceFactory, ServiceResponse},
    web, App,
};

use actix_session::config::PersistentSession;

use anyhow::Error;
use diesel::{pg::PgConnection, prelude::*, r2d2, r2d2::ConnectionManager};
use std::sync::Arc;
pub mod db;
pub mod endpoints;
pub mod errors;

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
    let master_key: &Vec<u8> = &(0..32).collect();
    let secret_key = Key::derive_from(master_key);

    App::new()
        .app_data(web::Data::new(pool))
        .wrap(
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .session_lifecycle(
                    PersistentSession::default().session_ttl(time::Duration::days(5)),
                )
                .build(),
            // SessionMiddleware::new(
            // CookieSessionStore::default(),
            // secret_key.clone(),
        )
        .configure(endpoints::scoped_config)
}
