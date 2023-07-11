use argon2::{
    password_hash::{Error, PasswordVerifier},
    PasswordHash,
};
use rand::{distributions::Alphanumeric, Rng};

use crate::models::User;

#[derive(serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn authorize_user(user: &User, credentials: &Credentials) -> Result<String, Error> {
    let db_hash = PasswordHash::new(&user.password)?;
    let argon = argon2::Argon2::default();

    argon.verify_password(credentials.password.as_bytes(), &db_hash)?;

    Ok(rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect())
}
