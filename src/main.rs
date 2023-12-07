use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use rocket::{Build, Rocket};
use rocket_codegen::{launch, routes};
use std::env;

mod db;
mod models;
mod schema;
mod static_files;

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set as an environment variable");

    let pool = db::init_pool(&database_url);
    rocket::build()
        .manage(pool)
        .mount("/", routes![static_files::index, static_files::all])
}
