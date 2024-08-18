use crate::models::Task;
use crate::task::TaskService;
use crate::{Delete, DeleteTools};
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{delete, prelude::*};

impl Delete<Task> for TaskService {
    fn delete(connection: &mut MysqlConnection, item: Task) -> Result<usize, Error> {
        use crate::schema::tasks;
        delete(tasks::table.find(item.id)).execute(connection)
    }
}

impl DeleteTools<Task> for TaskService {
    fn delete_by_id(connection: &mut MysqlConnection, id: i32) -> Result<usize, Error> {
        use crate::schema::tasks;
        delete(tasks::table)
            .filter(tasks::id.eq(id))
            .execute(connection)
    }
}
