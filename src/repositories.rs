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
                rustaceans::name.eq(data.email.to_owned()),
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
