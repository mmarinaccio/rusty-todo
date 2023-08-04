#[macro_use]
extern crate rocket;
extern crate diesel;

mod db;
mod models;
mod routes;
mod schema;
mod service;

use routes::{create_task, index};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, create_task])
}
