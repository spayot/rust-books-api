use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use std::env;

mod models;
mod schema;

fn main() {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set as an environment variable");

    let mut con = PgConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: "And Then There Were None".to_string(),
        author: "Agatha Christie".to_string(),
        published: true,
    };

    if models::Book::insert(book, &mut con) {
        println!("success!");
    } else {
        println!("failed :(");
    }
}
