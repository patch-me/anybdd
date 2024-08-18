use crate::models::NewTask;
use crate::task::TaskService;
use crate::Post;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{insert_into, prelude::*};

impl Post<NewTask> for TaskService {
    fn create(connection: &mut MysqlConnection, item: NewTask) -> Result<usize, Error> {
        use crate::schema::tasks;
        insert_into(tasks::table).values(item).execute(connection)
    }
}
