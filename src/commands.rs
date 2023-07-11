use diesel::{Connection, PgConnection};

use crate::{
    auth::hash_password,
    models::NewUser,
    repositories::{RoleRepository, UserRepository},
};

fn load_db_connexction() -> PgConnection {
    let db_url = std::env::var("DATABASE_URL").expect("Could not load DB url");
    PgConnection::establish(&db_url).expect("Could not connect to postgres DB")
}

pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut conn = load_db_connexction();

    let password_hash = hash_password(password).unwrap();
    let new_user = NewUser {
        username,
        password: password_hash,
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
