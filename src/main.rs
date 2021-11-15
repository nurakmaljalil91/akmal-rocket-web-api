#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
// use rocket::tokio::time::{sleep, Duration};
//
// mod schema;
// mod todo;
//
// fn hello_world(){
//     println!("Hello, world!");
// }

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

// #[get("/delay/<seconds>")]
// async fn delay(seconds: u64) -> String {
//     sleep(Duration::from_secs(seconds)).await;
//     format!("Waited for {} seconds", seconds)
// }
//
// #[rocket::main]
// async fn main() {
//     hello_world();
//     rocket::build()
//         .mount("/", routes![delay])
//         .launch().await;
// }

pub fn establish_connection() -> PgConnection{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[database("postgresql_database")]
pub struct DbConn(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    establish_connection();
    rocket::build().mount("/", routes![index])
}