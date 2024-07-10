#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use rocket_sync_db_pools::{database, diesel::MysqlConnection};
use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::http::Status;
use rocket::response::status;
use dotenv::dotenv;
use diesel::mysql::Mysql;

mod schema {
    table! {
        todos (id) {
            id -> Integer,
            title -> Varchar,
            description -> Nullable<Text>,
            done -> Bool,
        }
    }
}

use schema::todos;

#[database("mysql_db")]
struct DbConn(MysqlConnection);

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Clone)]
#[diesel(table_name = todos)]
struct Todo {
    id: Option<i32>,
    title: String,
    description: Option<String>,
    done: bool,
}

#[derive(Selectable, Queryable)]
#[diesel(table_name = todos)]
struct SelectableTodo {
    id: i32,
    title: String,
    description: Option<String>,
    done: bool,
}

#[post("/todo", data = "<todo>")]
async fn create_todo(conn: DbConn, todo: Json<Todo>) -> Result<Json<Todo>, status::Custom<&'static str>> {
    let new_todo = todo.into_inner();
    let new_todo_clone = new_todo.clone();
    conn.run(move |c| {
        diesel::insert_into(todos::table)
            .values(&new_todo)
            .execute(c)
    }).await.map_err(|_| status::Custom(Status::InternalServerError, "Error creating todo"))?;
    Ok(Json(new_todo_clone))
}

#[get("/todos")]
async fn get_todos(conn: DbConn) -> Result<Json<Vec<Todo>>, status::Custom<&'static str>> {
    let result = conn.run(|c| todos::table.select((todos::id, todos::title, todos::description, todos::done)).load::<SelectableTodo>(c)).await.map_err(|_| status::Custom(Status::InternalServerError, "Error loading todos"))?;
    Ok(Json(result.into_iter().map(|t| Todo { id: Some(t.id), title: t.title, description: t.description, done: t.done }).collect()))
}

#[put("/todo/<id>", data = "<todo>")]
async fn update_todo(conn: DbConn, id: i32, todo: Json<Todo>) -> Result<Json<Todo>, status::Custom<&'static str>> {
    let updated_todo = todo.into_inner();
    let updated_todo_clone = updated_todo.clone();
    conn.run(move |c| {
        diesel::update(todos::table.find(id))
            .set(&updated_todo)
            .execute(c)
    }).await.map_err(|_| status::Custom(Status::InternalServerError, "Error updating todo"))?;
    Ok(Json(updated_todo_clone))
}

#[delete("/todo/<id>")]
async fn delete_todo(conn: DbConn, id: i32) -> Result<&'static str, status::Custom<&'static str>> {
    conn.run(move |c| diesel::delete(todos::table.find(id)).execute(c)).await.map_err(|_| status::Custom(Status::InternalServerError, "Error deleting todo"))?;
    Ok("Todo deleted")
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![create_todo, get_todos, update_todo, delete_todo])
}
