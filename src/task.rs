use crate::models::{NewTask, Task};
use crate::schema::tasks;
use crate::{Delete, Get, Patch, Post};
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{delete, insert_into, prelude::*, update};

pub struct TaskService;

impl Get<Task> for TaskService {
    fn get(connection: &mut MysqlConnection) -> Result<Vec<Task>, Error> {
        // Implement logic to get all tasks here
        use crate::schema::tasks::dsl::*;
        tasks.load::<Task>(connection)
    }

    fn get_by_id(connection: &mut MysqlConnection, id_user: i32) -> Result<Vec<Task>, Error> {
        // Implement logic to get a user by id here
        use crate::schema::tasks;
        tasks::table
            .filter(crate::schema::tasks::id.eq(id_user))
            .load::<Task>(connection)
    }
}
impl Post<Task, NewTask> for TaskService {
    fn create(connection: &mut MysqlConnection, item: NewTask) -> Result<usize, Error> {
        use crate::schema::tasks;
        insert_into(tasks::table).values(item).execute(connection)
    }
}
impl Patch<Task> for TaskService {
    fn edit(connection: &mut MysqlConnection, item: Task) -> Result<usize, Error> {
        use crate::schema::tasks;
        update(tasks::table.find(item.id))
            .set(item)
            .execute(connection)
    }
}
impl Delete<Task> for TaskService {
    fn delete(connection: &mut MysqlConnection, item: Task) -> Result<usize, Error> {
        delete(tasks::table.find(item.id)).execute(connection)
    }
    fn delete_by_id(connection: &mut MysqlConnection, id: i32) -> Result<usize, Error> {
        delete(tasks::table)
            .filter(tasks::id.eq(id))
            .execute(connection)
    }
}
