use routes::rustaceans::*;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod models;
mod repositories;
mod routes;
mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                get_rustacean_by_id,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .launch()
        .await;
}
