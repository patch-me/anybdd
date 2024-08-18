use crate::models::UserProject;
use crate::user_project::UserProjectService;
use crate::Delete;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{delete, prelude::*};

impl Delete<UserProject> for UserProjectService {
    fn delete(connection: &mut MysqlConnection, item: UserProject) -> Result<usize, Error> {
        use crate::schema::user_projects;
        delete(user_projects::table.find((item.user_id, item.project_id))).execute(connection)
    }
}
