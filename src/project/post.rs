use crate::models::NewProject;
use crate::project::ProjectService;
use crate::user_project::UserProjectService;
use crate::Post;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{insert_into, prelude::*};

impl Post<NewProject> for ProjectService {
    fn create(connection: &mut MysqlConnection, item: NewProject) -> Result<usize, Error> {
        use crate::schema::projects;
        insert_into(projects::table)
            .values(item)
            .execute(connection)
    }
}
impl ProjectService {
    pub fn create(connection: &mut MysqlConnection, item: NewProject, users_id: Vec<i32>) {
        // Explicitly call the `create` method from the `Post<NewProject>` trait implementation
        <ProjectService as Post<NewProject>>::create(connection, item);
    }
}
