use rocket::serde::{Serialize, Deserialize};
use crate::schema::todo;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CustomResponse{
    pub status_code: i32,
    pub message: &'static str
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name="todo"]
pub struct Todo{
    pub id: i32,
    pub name: String,
    pub complete: bool
}

#[derive(Debug, FromForm)]
pub struct NewTodo {
    pub name: String
}