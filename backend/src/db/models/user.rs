use diesel::prelude::*;
use serde::Serialize;
// #![allow(unused)]
// #![allow(clippy::all)]

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::db::schema::user)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
}
