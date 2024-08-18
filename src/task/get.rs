use crate::models::Task;
use crate::task::TaskService;
use crate::Get;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

impl Get<Task> for TaskService {
    fn get(connection: &mut MysqlConnection) -> Result<Vec<Task>, Error> {
        // Implement logic to get all users here
        use crate::schema::tasks::dsl::*;
        tasks.load::<Task>(connection)
    }

    fn get_by_id(connection: &mut MysqlConnection, id_user: i32) -> Result<Vec<Task>, Error> {
        // Implement logic to get a user by id here
        use crate::schema::tasks;
        tasks::table
            .filter(tasks::id.eq(id_user))
            .load::<Task>(connection)
    }
}
