use crate::models::{NewTask, Task};
use crate::service::create_task_record;
use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/")]
pub fn index() -> &'static str {
    return "Hello, world!";
}

#[post("/tasks", format = "json", data = "<task>")]
pub fn create_task(task: Json<NewTask>) -> Result<Created<Json<Task>>> {
    let created_task: Task = create_task_record(task.into_inner());

    return Ok(Created::new("/tasks").body(Json(created_task)));
}
