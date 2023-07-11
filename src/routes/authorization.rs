use rocket::{
    response::status::Custom,
    serde::json::{serde_json::json, Json, Value},
};
use rocket_db_pools::Connection;

use crate::auth::{self, Credentials};
use crate::repositories::UserRepository;

use super::{server_error, Cache, DB};

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    db: DB,
    cache: Connection<Cache>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        UserRepository::get_by_username(c, &credentials.username)
            .map(|user| {
                if let Ok(token) = auth::authorize_user(&user, &credentials) {
                    return json!(token);
                }
                json!("Unauthorized")
            })
            .map_err(|e| server_error(&e.into()))
    })
    .await
}
