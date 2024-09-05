use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::project_tasks)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ProjectTask {
    pub project_id: i32,
    pub task_id: i32,
}
