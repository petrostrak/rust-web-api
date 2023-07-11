pub mod authorization;
pub mod crates;
pub mod rustaceans;

use diesel::PgConnection;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};
use rocket_db_pools::{deadpool_redis, Database};
use rocket_sync_db_pools::database;

#[rocket_sync_db_pools::database("postgres")]
pub struct DB(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct Cache(deadpool_redis::Pool);

fn server_error(e: &Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Something went wrong"))
}
