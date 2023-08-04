use crate::models::{NewTask, Task};
use crate::service::{create_task_record, delete_task_record, get_task_records};
use rocket::response::{status::Created, status::NoContent, Debug};
use rocket::serde::json::Json;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/")]
pub fn index() -> &'static str {
    return "Hello, world!";
}

#[get("/tasks")]
pub fn get_tasks() -> Json<Vec<Task>> {
    let tasks: Vec<Task> = get_task_records();

    return Json(tasks);
}

#[post("/tasks", format = "json", data = "<task>")]
pub fn create_task(task: Json<NewTask>) -> Result<Created<Json<Task>>> {
    let created_task: Task = create_task_record(task.into_inner());

    return Ok(Created::new("/tasks").body(Json(created_task)));
}

#[delete("/tasks/<id>")]
pub fn delete_task(id: i32) -> NoContent {
    delete_task_record(id);

    return NoContent;
}
