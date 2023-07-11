use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{serde_json::json, Json, Value},
};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};

use crate::auth::{self, Credentials};
use crate::repositories::UserRepository;

use super::{server_error, Cache, DB};

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    db: DB,
    mut cache: Connection<Cache>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    let username = credentials.username.clone();
    let user = db
        .run(move |c| {
            UserRepository::get_by_username(c, &username).map_err(|e| server_error(&e.into()))
        })
        .await?;

    let session_id = auth::authorize_user(&user, &credentials)
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong credentials")))?;

    cache
        .set_ex::<_, _, ()>(format!("sessions/{}", session_id), user.id, 3 * 60 * 60)
        .await
        .map(|_| json!({ "token": session_id }))
        .map_err(|e| server_error(&e.into()))
}
