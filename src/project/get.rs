use crate::models::Project;
use crate::project::ProjectService;
use crate::Get;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

impl Get<Project> for ProjectService {
    fn get(connection: &mut MysqlConnection) -> Result<Vec<Project>, Error> {
        // Implement logic to get all projects here
        use crate::schema::projects::dsl::*;
        projects.load::<Project>(connection)
    }

    fn get_by_id(connection: &mut MysqlConnection, id_user: i32) -> Result<Vec<Project>, Error> {
        // Implement logic to get a user by id here
        use crate::schema::projects;
        projects::table
            .filter(crate::schema::projects::id.eq(id_user))
            .load::<Project>(connection)
    }
}
