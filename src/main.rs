use anybdd::connection::establish_connection;
use anybdd::models::Project;
use anybdd::schema::projects::dsl::*;
use diesel::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut connection = establish_connection();

    let results: Vec<Project> = projects
        .select(Project::as_select())
        .load(&mut connection)
        .expect("Error loading projects");

    println!("{:?}", results);

    Ok(())
}
