#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

// mod auth;
mod models;
mod schema;

use rocket::response::status;
use rocket::serde::json::serde_json::{Value, json};
use rocket::serde::json::Json;
use rocket::form::Form;
// use auth::BasicAuth;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::schema::todo::dsl::*;
use crate::schema::todo;
use crate::models::{CustomResponse, NewTodo, Todo};


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
fn get_todo() -> Json<Vec<Todo>> {
    let connection = establish_connection();


    let all = todo.limit(100).load::<Todo>(&connection)
        .expect("Error loading todo");

    Json(all)
}

#[get("/todo/<other_id>")]
fn get_single_todo(other_id: i32) -> Json<Todo> {
    let connection = establish_connection();

    let single_todo = todo
        .filter(id.eq(other_id))
        .first(&connection)
        .expect("Error loading todo");


    Json(single_todo)
}

#[post("/todo", data = "<todo_form>")]
fn create_todo(todo_form: Json<NewTodo>) -> Json<CustomResponse> {
    let connection = establish_connection();

    diesel::insert_into(todo::table)
        .values(&*todo_form)
        .execute(&connection)
        .expect("Error saving new todo");

    let response = CustomResponse {
        status_code: 200,
        message: "Insert successful!",
    };
    Json(response)
}

// #[put("/todo/<id>", format = "json")]
// fn update_todo(id: i32) -> Value {
//     json!({
// "id": id,
// "description": "Deploy personal web app ",
// "complete": true,
// "createdDate": "2020-11-19T16:16:18.383904+08:00"
// })
// }


#[delete("/todo")]
fn delete_todo() -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Json<CustomResponse> {
    let response = CustomResponse {
        status_code: 404,
        message: "Route Not Found!",
    };

    Json(response)
}

#[catch(401)]
fn unauthorized() -> Json<CustomResponse> {
    let response = CustomResponse {
        status_code: 401,
        message: "Unauthorized!",
    };

    Json(response)
}

#[launch]
fn rocket() -> _ {
    // establish_connection();
    rocket::build().mount("/api/v1", routes![
        hello,
        get_todo,
        get_single_todo,
        create_todo,
        delete_todo
    ]).register("/", catchers![not_found,unauthorized])
}