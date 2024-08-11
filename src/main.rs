pub mod models;
pub mod schema;
use anybdd::schema::projects;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::Project;
use std::env;

fn main() {
    use self::schema::projects::dsl::*;
    let connection = &mut establish_connection();

    let results = projects
        .select(Project::as_select())
        .load(connection)
        .expect("Error loading projects");
}
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
