use diesel::{prelude::*, sql_types::Serial};
use serde::Serialize;
// #![allow(unused)]
// #![allow(clippy::all)]

#[derive(Queryable, Insertable, Selectable, Debug, Serialize)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::db::schema::user)]
pub struct User {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: String,
}
