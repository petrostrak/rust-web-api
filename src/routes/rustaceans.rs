use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{serde_json::json, Json, Value};

use crate::models::{NewRustacean, Rustacean, User};
use crate::repositories::RustaceanRepository;
use crate::routes::{server_error, DB};

use super::EditorUser;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(db: DB, _user: User) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::get_all(c, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| server_error(&e.into()))
    })
    .await
}

#[rocket::get("/rustaceans/<id>")]
pub async fn get_rustacean_by_id(db: DB, id: i32, _user: User) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::get_by_id(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| server_error(&e.into()))
    })
    .await
}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    db: DB,
    new_rustacean: Json<NewRustacean>,
    _user: EditorUser,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| Custom(Status::Created, json!(rustacean)))
            .map_err(|e| server_error(&e.into()))
    })
    .await
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    db: DB,
    id: i32,
    rustacean: Json<Rustacean>,
    _user: EditorUser,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::update(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| server_error(&e.into()))
    })
    .await
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    db: DB,
    id: i32,
    _user: EditorUser,
) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e| server_error(&e.into()))
    })
    .await
}
