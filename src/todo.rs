use rocket::serde::Serialize;
use diesel::{self, result::QueryResult, prelude::*};

use schema::todo;
use schema::todo::dsl::{tasks as all_tasks, completed as task_completed};

#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name="todo"]
pub struct Todo {
    pub id: Option<i32>,
    pub description: String,
    pub complete: bool
}

#[derive(Debug, FromForm)]
pub struct Task{
    pub description: String,
}

imp Todo {
    pub async fn all()
}
