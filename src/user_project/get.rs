use crate::models::UserProject;
use crate::user_project::UserProjectService;
use crate::Get;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

impl Get<UserProject> for UserProjectService {
    fn get(connection: &mut MysqlConnection) -> Result<Vec<UserProject>, Error> {
        // Implement logic to get all user_projects here
        use crate::schema::user_projects::dsl::*;
        user_projects.load::<UserProject>(connection)
    }

    fn get_by_id(
        connection: &mut MysqlConnection,
        id_user: i32,
    ) -> Result<Vec<UserProject>, Error> {
        // Implement logic to get a user by id here
        use crate::schema::user_projects;
        user_projects::table
            .filter(crate::schema::user_projects::user_id.eq(id_user))
            .load::<UserProject>(connection)
    }
}
