use argon2::PasswordHash;
use rocket::{
    response::status::Custom,
    serde::json::{serde_json::json, Json, Value},
};

use crate::repositories::UserRepository;

use super::{server_error, DB};

#[derive(serde::Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(db: DB, credentials: Json<Credentials>) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        UserRepository::get_by_username(c, &credentials.username)
            .map(|user| {
                let db_hash = PasswordHash::new(&user.password).unwrap();
                let argon = argon2::Argon2::default();

                use argon2::password_hash::PasswordVerifier;
                if argon
                    .verify_password(credentials.password.as_bytes(), &db_hash)
                    .is_ok()
                {
                    return json!("Success");
                }
                json!("Unauthorized")
            })
            .map_err(|e| server_error(&e.into()))
    })
    .await
}
