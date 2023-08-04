#[macro_use]
extern crate rocket;
extern crate diesel;

mod db;
mod models;
mod routes;
mod schema;
mod service;
use routes::{create_task, delete_task, get_tasks, index};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::pg::Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    run_migrations(&mut db::establish_connection()).unwrap();

    rocket::build().mount("/", routes![index, get_tasks, create_task, delete_task])
}
