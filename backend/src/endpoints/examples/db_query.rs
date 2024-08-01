use crate::models::User;
use crate::schema::user::dsl::*;
use crate::DB;
use actix_web::{error, web, HttpResponse, Responder};
use derive_more::{Display, Error};
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2,
    r2d2::{ConnectionManager, PooledConnection},
};

use serde_json::json;
use std::fmt::format;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", pesho)]
struct CustomError {
    pesho: &'static str,
}

impl error::ResponseError for CustomError {}

pub async fn simple_query(
    mut pool: web::Data<DB>,
) -> Result<impl Responder, impl error::ResponseError> {
    let mut a: PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().map_err(|e| CustomError { pesho: "gosho" })?;
    let b = a.deref_mut();
    let result = user
        .limit(5)
        .select(User::as_select())
        .load(b)
        .expect("some error");

    let result = json!(result);

    Ok::<HttpResponse, CustomError>(HttpResponse::Ok().body(format!("Hey there! {:?}", result)))
}
