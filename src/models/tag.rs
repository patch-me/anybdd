use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Tag {
    pub id: i32,
    pub name: String,
}
#[derive(Insertable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewTag {
    pub name: String,
}
