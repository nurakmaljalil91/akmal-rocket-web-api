
#[macro_use]
extern crate rocket;
// #[macro_use] extern crate diesel;
// #[macro_use] extern crate dotenv;

mod auth;


use rocket::response::status;
use rocket::serde::json::serde_json::{Value, json};
use auth::BasicAuth;
// use diesel::prelude::*;
// use diesel::pg::PgConnection;
// use dotenv::dotenv;
// use std::env;
// use rocket::tokio::time::{sleep, Duration};
//
// mod schema;
// mod todo;
//
// fn hello_world(){
//     println!("Hello, world!");
// }



// #[get("/")]
// fn index() -> &'static str {
//     "Hello World"
// }

// #[get("/delay/<seconds>")]
// async fn delay(seconds: u64) -> String {
//     sleep(Duration::from_secs(seconds)).await;
//     format!("Waited for {} seconds", seconds)
// }
//

#[get("/")]
fn hello() -> Value {
    json!("Hello World")
}
// #[rocket::main]
// async fn main() {
//     let _ = rocket::build()
//         .mount("/", routes![hello])
//         .launch().await;
// }

// pub fn establish_connection() -> PgConnection{
//     dotenv().ok();
//
//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//
//     PgConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }

// #[database("postgresql_database")]
// pub struct DbConn(diesel::PgConnection);
//

#[get("/todo")]
fn get_todo(_auth: BasicAuth) -> Value {
    json!([{
"id": "11883816-d6d7-4d9a-95ad-cea83930413e",
"description": "Testing todo controller before publish",
"complete": true,
"createdDate": "2020-11-18T16:22:08.304015+08:00"
},
{
"id": "a94a5beb-7439-4123-9c8c-c83ee964b801",
"description": "Update Mr Ang and Mr Ralph about db_ms storage size",
"complete": true,
"createdDate": "2020-11-18T16:02:28.141744+08:00"
},
{
"id": "4dddc472-38f2-422d-b95a-3d464664a054",
"description": "Deploy personal web app ",
"complete": true,
"createdDate": "2020-11-19T16:16:18.383904+08:00"
}])
}

#[get("/todo/<id>")]
fn get_single_todo(id: i32) -> Value {
    json!({
"id": id,
"description": "Testing todo controller before publish",
"complete": true,
"createdDate": "2020-11-18T16:22:08.304015+08:00"
})
}

#[post("/todo", format = "json")]
fn create_todo() -> Value {
    json!({
"id": "4dddc472-38f2-422d-b95a-3d464664a054",
"description": "Deploy personal web app ",
"complete": true,
"createdDate": "2020-11-19T16:16:18.383904+08:00"
})
}

#[put("/todo/<id>", format = "json")]
fn update_todo(id: i32) -> Value {
    json!({
"id": id,
"description": "Deploy personal web app ",
"complete": true,
"createdDate": "2020-11-19T16:16:18.383904+08:00"
})
}


#[delete("/todo")]
fn delete_todo() -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value{
    json!("Not found")
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("Invalid/Missing authorization")
}

#[launch]
fn rocket() -> _ {
    // establish_connection();
    rocket::build().mount("/", routes![
        hello,
        get_todo,
        get_single_todo,
        create_todo,
        update_todo,
        delete_todo
    ]).register("/", catchers![not_found,unauthorized])
}