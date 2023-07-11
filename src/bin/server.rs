use rocket_db_pools::Database;

extern crate rustycrates;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                rustycrates::routes::rustaceans::get_rustaceans,
                rustycrates::routes::rustaceans::get_rustacean_by_id,
                rustycrates::routes::rustaceans::create_rustacean,
                rustycrates::routes::rustaceans::update_rustacean,
                rustycrates::routes::rustaceans::delete_rustacean,
                rustycrates::routes::crates::get_crates,
                rustycrates::routes::crates::get_crate_by_id,
                rustycrates::routes::crates::create_crate,
                rustycrates::routes::crates::update_crate,
                rustycrates::routes::crates::delete_crate,
                rustycrates::routes::authorization::login
            ],
        )
        .attach(rustycrates::routes::DB::fairing())
        .attach(rustycrates::routes::Cache::init())
        .launch()
        .await;
}
