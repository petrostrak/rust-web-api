use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn get_all(conn: &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table
            .limit(limit)
            .order(rustaceans::id.desc())
            .load::<Rustacean>(conn)
    }

    pub fn get_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result::<Rustacean>(conn)
    }

    pub fn create(conn: &mut PgConnection, data: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(data)
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, id: i32, data: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(data.name.to_owned()),
                rustaceans::email.eq(data.email.to_owned()),
            ))
            .execute(conn)?;
        Self::get_by_id(conn, id)
    }

    pub fn delete(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(conn)
    }
}

pub struct CrateRepository;

impl CrateRepository {
    pub fn get_all(conn: &mut PgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table
            .limit(limit)
            .order(crates::id.desc())
            .load::<Crate>(conn)
    }

    pub fn get_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result::<Crate>(conn)
    }

    pub fn create(conn: &mut PgConnection, data: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(data)
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, id: i32, data: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::rustacean_id.eq(data.rustacean_id.to_owned()),
                crates::code.eq(data.code.to_owned()),
                crates::name.eq(data.name.to_owned()),
                crates::version.eq(data.version.to_owned()),
                crates::description.eq(data.description.to_owned()),
            ))
            .execute(conn)?;
        Self::get_by_id(conn, id)
    }

    pub fn delete(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(conn)
    }
}

pub struct UserRepository;

impl UserRepository {
    pub fn get_all(conn: &mut PgConnection, limit: i64) -> QueryResult<Vec<User>> {
        users::table
            .limit(limit)
            .order(users::id.desc())
            .load::<User>(conn)
    }

    pub fn create(
        conn: &mut PgConnection,
        data: NewUser,
        role_codes: Vec<String>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(data)
            .get_result::<User>(conn)?;

        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::get_by_code(conn, &role_code) {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    let new_role = NewRole {
                        code: role_code.to_owned(),
                        name: role_code.to_owned(),
                    };
                    let role = RoleRepository::create(conn, new_role)?;
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                }
            };
            diesel::insert_into(users_roles::table)
                .values(new_user_role)
                .execute(conn)?;
        }
        Ok(user)
    }

    pub fn delete(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(conn)
    }
}

pub struct RoleRepository;

impl RoleRepository {
    pub fn get_by_user(conn: &mut PgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(user).get_results(conn)?;
        Self::get_by_ids(
            conn,
            user_roles
                .iter()
                .map(|user_role: &UserRole| user_role.role_id)
                .collect(),
        )
    }

    pub fn get_by_ids(conn: &mut PgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table
            .filter(roles::id.eq_any(ids))
            .get_results::<Role>(conn)
    }

    pub fn get_by_code(conn: &mut PgConnection, code: &String) -> QueryResult<Role> {
        roles::table
            .filter(roles::code.eq(code))
            .get_result::<Role>(conn)
    }

    pub fn create(conn: &mut PgConnection, data: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(data)
            .get_result(conn)
    }
}
