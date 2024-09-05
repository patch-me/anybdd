use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::task_tags)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TaskTag {
    pub task_id: i32,
    pub tag_id: i32,
}
