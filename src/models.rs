use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Crate {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crates)]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}
