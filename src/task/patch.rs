use crate::models::Task;
use crate::task::TaskService;
use crate::Patch;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{prelude::*, update};

impl Patch<Task> for TaskService {
    fn edit(connection: &mut MysqlConnection, item: Task) -> Result<usize, Error> {
        use crate::schema::tasks;
        update(tasks::table.find(item.id))
            .set(item)
            .execute(connection)
    }
}
