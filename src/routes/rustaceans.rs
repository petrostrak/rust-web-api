use rocket::serde::json::{serde_json::json, Value};

#[get("/rustaceans")]
pub fn get_rustaceans() -> Value {
    json!([])
}

#[get("/rustaceans/<id>")]
pub fn get_rustacean_by_id(id: i32) {}

#[post("/rustaceans")]
pub fn create_rustacean() {}

#[put("/rustacean/<id>")]
pub fn update_rustacean(id: i32) {}

#[delete("/rustacean/<id>")]
pub fn delete_rustacean(id: i32) {}
