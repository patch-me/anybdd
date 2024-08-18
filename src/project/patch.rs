use crate::models::Project;
use crate::project::ProjectService;
use crate::Patch;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{prelude::*, update};

impl Patch<Project> for ProjectService {
    fn edit(connection: &mut MysqlConnection, item: Project) -> Result<usize, Error> {
        use crate::schema::projects;
        update(projects::table.find(item.id))
            .set(item)
            .execute(connection)
    }
}
