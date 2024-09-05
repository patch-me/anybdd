use diesel::prelude::*;

/// Represents a user-project relationship in the database.
#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::user_projects)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserProject {
    pub user_id: i32,
    pub project_id: i32,
}
