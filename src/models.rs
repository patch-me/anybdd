use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user_projects)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserProject {
    pub user_id: i32,
    pub project_id: i32,
}

#[derive(Debug)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub due_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::project_tasks)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ProjectTask {
    pub project_id: i32,
    pub task_id: i32,
}

#[derive(Debug)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user_tasks)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserTask {
    pub user_id: i32,
    pub task_id: i32,
}

#[derive(Debug)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Debug)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::task_tags)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TaskTag {
    pub task_id: i32,
    pub tag_id: i32,
}
