#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;

mod auth;
mod models;
mod schema;

use rocket::response::status;
use rocket::serde::json::serde_json::{Value, json};
use auth::BasicAuth;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::types::Json;
use crate::schema::todo::dsl::todo;
use crate::models::Todo;
// use crate::schema::todo::dsl::todo;


#[get("/")]
fn hello() -> Value {
    json!(
        {
            "name": "Akmal Web API with Rust",
            "version": 0.1
        })
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/todo")]
fn get_todo() -> Value {

    let connection = establish_connection();


        let all = todo.limit(100).load::<Todo>(&connection)
            .expect("Error loading todo");
        json!(all)


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
fn not_found() -> Value {
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