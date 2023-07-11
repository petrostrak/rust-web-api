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
    pub fn get_by_id(c: &mut PgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(c)
    }

    pub fn get_by_username(conn: &mut PgConnection, username: &String) -> QueryResult<User> {
        users::table
            .filter(users::username.eq(username))
            .get_result::<User>(conn)
    }

    pub fn get_with_roles(
        conn: &mut PgConnection,
    ) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load::<User>(conn)?;
        let result = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(conn)?
            .grouped_by(&users);
        Ok(users.into_iter().zip(result).collect())
    }

    pub fn create(
        c: &mut PgConnection,
        new_user: NewUser,
        role_codes: Vec<RoleCode>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(c)?;

        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::get_by_code(c, &role_code) {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    let name = role_code.to_string();
                    let new_role = NewRole {
                        name,
                        code: role_code,
                    };
                    let role = RoleRepository::create(c, new_role)?;
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                }
            };

            diesel::insert_into(users_roles::table)
                .values(new_user_role)
                .get_result::<UserRole>(c)?;
        }

        Ok(user)
    }

    pub fn delete(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users_roles::table.filter(users_roles::user_id.eq(id))).execute(conn)?;
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

    pub fn get_by_code(conn: &mut PgConnection, code: &RoleCode) -> QueryResult<Role> {
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
