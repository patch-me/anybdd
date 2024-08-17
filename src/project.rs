use crate::models::{NewProject, Project};
use crate::schema::projects;
use crate::{Delete, Get, Patch, Post};
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{delete, insert_into, prelude::*, update};

pub struct ProjectService;

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
impl Post<Project, NewProject> for ProjectService {
    fn create(connection: &mut MysqlConnection, item: NewProject) -> Result<usize, Error> {
        use crate::schema::projects;
        insert_into(projects::table)
            .values(item)
            .execute(connection)
    }
}
impl Patch<Project> for ProjectService {
    fn edit(connection: &mut MysqlConnection, item: Project) -> Result<usize, Error> {
        use crate::schema::projects;
        update(projects::table.find(item.id))
            .set(item)
            .execute(connection)
    }
}
impl Delete<Project> for ProjectService {
    fn delete(connection: &mut MysqlConnection, item: Project) -> Result<usize, Error> {
        delete(projects::table.find(item.id)).execute(connection)
    }
    fn delete_by_id(connection: &mut MysqlConnection, id: i32) -> Result<usize, Error> {
        delete(projects::table)
            .filter(projects::id.eq(id))
            .execute(connection)
    }
}
