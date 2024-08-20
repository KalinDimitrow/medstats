use crate::db::models::user::User; //user::dsl::*;
use crate::db::schema::user;
use crate::db::schema::user::dsl::*;
use crate::errors::Error;
use crate::DB;
use actix_web::{error, web, HttpResponse, Responder};
use derive_more::{Display, Error};
use diesel::prelude::*;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
};

use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use serde_json::json;
use std::fmt::format;
use std::ops::{Deref, DerefMut};

pub async fn create_user(
    mut pool: web::Data<DB>,
    session: Session,
) -> Result<impl Responder, impl error::ResponseError> {
    let mut a: PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().map_err(|e| Error {
            status_code: 10,
            message: String::from(" something"),
        })?;
    let b = a.deref_mut();

    let new_user = User {
        id: 1,
        name: String::from("someone"),
        password: String::from("password or something"),
    };

    let inserted_user = diesel::insert_into(user::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(b)
        .expect("Error saving new post");

    let res = session.insert("id", new_user.id);
    match res {
        Err(error) => println!("session error {:?}", error),
        _ => println!("No session error"),
    }
    // session.renew();

    Ok::<HttpResponse, Error>(
        HttpResponse::Ok().body(format!("Hey there!!!!!! {:?}", inserted_user)),
    )
}

pub async fn test_user(
    mut pool: web::Data<DB>,
    session: Session,
) -> Result<impl Responder, impl error::ResponseError> {
    let user_id = session.get::<i32>("id")?;
    let user_id = user_id.ok_or(Error {
        status_code: 500,
        message: "No such user".to_string(),
    })?;
    let mut connection = pool.get()?;

    let b = connection.deref_mut();
    let gosho = user
        .filter(user::id.eq(user_id))
        .first::<crate::db::models::user::User>(b)?;

    let result = json!(gosho);

    Ok::<HttpResponse, Error>(HttpResponse::Ok().body(format!("Hey there!!!!!! {:?}", result)))
}
