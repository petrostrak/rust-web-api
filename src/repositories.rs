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
