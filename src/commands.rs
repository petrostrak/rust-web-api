use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    PasswordHasher,
};
use diesel::{Connection, PgConnection};

use crate::{
    models::NewUser,
    repositories::{RoleRepository, UserRepository},
};

fn load_db_connexction() -> PgConnection {
    let db_url = std::env::var("DATABASE_URL").expect("Could not load DB url");
    PgConnection::establish(&db_url).expect("Could not connect to postgres DB")
}

pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut conn = load_db_connexction();

    let salt = SaltString::generate(OsRng);
    let argon = argon2::Argon2::default();
    let hashed_password = argon.hash_password(password.as_bytes(), &salt).unwrap();
    let new_user = NewUser {
        username,
        password: hashed_password.to_string(),
    };

    let user = UserRepository::create(&mut conn, new_user, role_codes).unwrap();
    println!("User created: {:?}", user);
    let roles = RoleRepository::get_by_user(&mut conn, &user).unwrap();
    println!("With roles: {:?}", roles);
}

pub fn list_users() {
    let mut conn = load_db_connexction();
    let users = UserRepository::get_with_roles(&mut conn).unwrap();

    for user in users {
        println!("{:?}", user)
    }
}

pub fn delete_user(id: i32) {
    let mut conn = load_db_connexction();
    UserRepository::delete(&mut conn, id).unwrap();
    println!("User with id {} deleted successfully.", id)
}
