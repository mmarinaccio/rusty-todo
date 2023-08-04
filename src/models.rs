use crate::schema::tasks;
use diesel::{AsChangeset, Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Debug, Queryable, AsChangeset, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub completed: bool,
}
