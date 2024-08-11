use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
}
