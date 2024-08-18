use crate::models::Project;
use crate::project::ProjectService;
use crate::{Delete, DeleteTools};
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{delete, prelude::*};

impl Delete<Project> for ProjectService {
    fn delete(connection: &mut MysqlConnection, item: Project) -> Result<usize, Error> {
        use crate::schema::projects;
        delete(projects::table.find(item.id)).execute(connection)
    }
}

impl DeleteTools<Project> for ProjectService {
    fn delete_by_id(connection: &mut MysqlConnection, id: i32) -> Result<usize, Error> {
        use crate::schema::projects;
        delete(projects::table)
            .filter(projects::id.eq(id))
            .execute(connection)
    }
}
