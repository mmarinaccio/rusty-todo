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

    let inserted_row = diesel::insert_into(tasks)
        .values(&new_task)
        .get_result::<Task>(connection)
        .expect("Error saving new task");

    return inserted_row;
}
