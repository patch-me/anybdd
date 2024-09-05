use diesel::prelude::*;

/// Represents a tag in the database.
#[derive(Debug, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

/// Represents a new tag to be inserted into the database.
#[derive(Insertable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewTag {
    pub name: String,
}
