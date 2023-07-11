use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{serde_json::json, Json, Value};

use crate::models::{Crate, NewCrate, User};
use crate::repositories::CrateRepository;
use crate::routes::{server_error, DB};

#[rocket::get("/crates")]
pub async fn get_crates(db: DB, _user: User) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        CrateRepository::get_all(c, 100)
            .map(|crates| json!(crates))
            .map_err(|e| server_error(&e.into()))
    })
    .await
}

#[rocket::get("/crates/<id>")]
pub async fn get_crate_by_id(db: DB, id: i32, _user: User) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::get_by_id(c, id)
            .map(|crt| json!(crt))
            .map_err(|e| server_error(&e.into()))
    })
    .await
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    db: DB,
    new_crate: Json<NewCrate>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(|c| {
        CrateRepository::create(c, new_crate.into_inner())
            .map(|crt| Custom(Status::Created, json!(crt)))
            .map_err(|e| server_error(&e.into()))
    })
    .await
}

#[rocket::put("/crates/<id>", format = "json", data = "<crt>")]
pub async fn update_crate(
    db: DB,
    id: i32,
    crt: Json<Crate>,
    _user: User,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::update(c, id, crt.into_inner())
            .map(|crt| json!(crt))
            .map_err(|e| server_error(&e.into()))
    })
    .await
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(db: DB, id: i32, _user: User) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e| server_error(&e.into()))
    })
    .await
}
