use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::user_tasks)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserTask {
    pub user_id: i32,
    pub task_id: i32,
}
