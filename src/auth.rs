use argon2::{
    password_hash::{Error, PasswordVerifier, SaltString},
    Argon2, PasswordHash, PasswordHasher,
};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};

use crate::models::User;

#[derive(serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn authorize_user(user: &User, credentials: &Credentials) -> Result<String, Error> {
    let db_hash = PasswordHash::new(&user.password)?;
    let argon = Argon2::default();

    argon.verify_password(credentials.password.as_bytes(), &db_hash)?;

    Ok(rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect())
}

pub fn hash_password(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(OsRng);
    let argon = Argon2::default();
    let hashed_password = argon.hash_password(password.as_bytes(), &salt)?;
    Ok(hashed_password.to_string())
}
