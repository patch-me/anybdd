use crate::models::UserProject;
use crate::user_project::UserProjectService;
use crate::Post;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{insert_into, prelude::*};

impl Post<UserProject> for UserProjectService {
    fn create(connection: &mut MysqlConnection, item: UserProject) -> Result<usize, Error> {
        use crate::schema::user_projects;
        insert_into(user_projects::table)
            .values(item)
            .execute(connection)
    }
}
