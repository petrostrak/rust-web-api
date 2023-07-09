use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::Json,
};
use serde_json::{json, Value};

use crate::{
    models::{Crate, NewCrate},
    repositories::CrateRepository,
    routes::server_error,
    DB,
};

#[get("/crates")]
pub async fn get_crates(db: DB) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        CrateRepository::get_all(c, 100)
            .map(|crates| json!(crates))
            .map_err(|e| server_error(&e.into()))
    })
    .await
}

#[get("/crates/<id>")]
pub async fn get_crate_by_id(db: DB, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::get_by_id(c, id)
            .map(|crt| json!(crt))
            .map_err(|e| server_error(&e.into()))
    })
    .await
}

#[post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    db: DB,
    new_crate: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(|c| {
        CrateRepository::create(c, new_crate.into_inner())
            .map(|crt| Custom(Status::Created, json!(crt)))
            .map_err(|e| server_error(&e.into()))
    })
    .await
}

#[put("/crates/<id>", format = "json", data = "<crt>")]
pub async fn update_crate(db: DB, id: i32, crt: Json<Crate>) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::update(c, id, crt.into_inner())
            .map(|crt| json!(crt))
            .map_err(|e| server_error(&e.into()))
    })
    .await
}

#[delete("/crates/<id>")]
pub async fn delete_crate(db: DB, id: i32) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e| server_error(&e.into()))
    })
    .await
}
