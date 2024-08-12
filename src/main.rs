pub mod models;
pub mod schema;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::Project;
use std::error::Error;
use std::env;
use crate::schema::projects::dsl::*; // Import the `projects` table and associated DSL items


fn main() -> Result<(), Box<dyn Error>> {
    let mut connection = establish_connection()?; // Handle the Result and unwrap the MysqlConnection

    let results: Vec<Project> = projects
        .select(Project::as_select())
        .load(&mut connection)
        .expect("Error loading projects");
    
    println!("{:?}", results);

    Ok(())
}

pub fn establish_connection() -> Result<MysqlConnection, Box<dyn Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    // Use ? instead of expect to propagate the error
    let connection = MysqlConnection::establish(&database_url)
        .map_err(|e| format!("Error connecting to {}: {}", database_url, e))?;
    
    Ok(connection)
}
