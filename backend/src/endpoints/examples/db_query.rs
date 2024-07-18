use std::fmt::format;
use std::ops::{Deref, DerefMut};
use actix_web::{web, HttpResponse, Responder, error};
use derive_more::{Display, Error};
use diesel::{pg::PgConnection, r2d2, r2d2::{ConnectionManager, PooledConnection}, prelude::*};
use crate::DB;

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
struct CustomError {
    name: &'static str,
}

impl error::ResponseError for CustomError {}

pub async fn simple_query(mut pool: web::Data<DB>) -> Result<impl Responder, impl error::ResponseError> {
    // let mut a = pool.acquire().await.map_err(|e| CustomError{name: "gosho"})?;
    // let a = pool.get().map_err(|e| CustomError{name: "gosho"})?;

    let mut a : PooledConnection<ConnectionManager<PgConnection>> = pool.get().map_err(|e| CustomError{name: "gosho"})?;
    let b = a.deref_mut();

    let users = diesel::sql_query("SELECT COUNT(*) FROM users").execute(b).map_err(|e| CustomError{name: "pesho"})?;



    // let b = a.acquire().await.map_err(|e| CustomError{name: "pesho"})?;
    // let b = a.server_version_num().unwrap();
    // let query = query("SELECT COUNT(*) FROM users")
    //     .bind(0)
    //     .fetch_one(b).await
    //     .map_err(|e| {
    //         println!("{:?}", e);
    //         CustomError{name: "atanas"}
    //     })?;
    
    Ok::<HttpResponse, CustomError>(HttpResponse::Ok().body(format!("Hey there! {}", users)))
}