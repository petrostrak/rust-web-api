use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{serde_json::json, Json, Value},
};

use crate::{
    models::{NewRustacean, Rustacean},
    repositories::RustaceanRepository,
    DB,
};

#[get("/rustaceans")]
pub async fn get_rustaceans(db: DB) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::get_all(c, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| Custom(Status::NotFound, json!(e.to_string())))
    })
    .await
}

#[get("/rustaceans/<id>")]
pub async fn get_rustacean_by_id(db: DB, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::get_by_id(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::NotFound, json!(e.to_string())))
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    db: DB,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::NotFound, json!(e.to_string())))
    })
    .await
}

#[put("/rustacean/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    db: DB,
    id: i32,
    rustacean: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::update(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::NotFound, json!(e.to_string())))
    })
    .await
}

#[delete("/rustacean/<id>")]
pub async fn delete_rustacean(db: DB, id: i32) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e| Custom(Status::NotFound, json!(e.to_string())))
    })
    .await
}
