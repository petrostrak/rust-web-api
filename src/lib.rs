#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use routes::DB;

mod models;
mod repositories;
pub mod routes;
mod schema;
