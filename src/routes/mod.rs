pub mod crates;
pub mod rustaceans;

use diesel::PgConnection;
use rocket::{http::Status, response::status::Custom};
use rocket_sync_db_pools::database;
use serde_json::{json, Value};
use std::error::Error;

#[database("postgres")]
pub struct DB(PgConnection);

fn server_error(e: &Box<dyn Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Something went wrong"))
}
