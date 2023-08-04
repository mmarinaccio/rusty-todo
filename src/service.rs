use crate::db::establish_connection;
use crate::models::{NewTask, Task};
use diesel::prelude::*;

pub fn create_task_record(task: NewTask) -> Task {
    use crate::schema::tasks::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection();

    let new_task = NewTask {
        title: &task.title,
        body: &task.body,
    };

    let inserted_row: Task = diesel::insert_into(tasks)
        .values(&new_task)
        .get_result::<Task>(connection)
        .expect("Error saving new task");

    return inserted_row;
}

pub fn get_task_records() -> Vec<Task> {
    use crate::schema::tasks::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection();

    let results: Vec<Task> = tasks.load::<Task>(connection).expect("Error loading tasks");

    return results;
}

pub fn delete_task_record(task_id: i32) {
    use crate::schema::tasks::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection();

    diesel::delete(tasks.find(task_id))
        .execute(connection)
        .expect("Error deleting task");
}
