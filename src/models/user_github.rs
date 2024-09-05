use diesel::prelude::*;

/// Represents a user's GitHub information.
#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::user_github)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserGithub {
    pub user_id: i32,
    pub github_token: String,
}
