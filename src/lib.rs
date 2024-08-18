pub mod connection;
pub mod models;
pub mod project;
pub mod schema;
pub mod tag;
pub mod task;
pub mod user;
pub mod user_project;
use std::usize;

use diesel::mysql::MysqlConnection;
use diesel::result::Error;

pub trait Get<T> {
    fn get(connection: &mut MysqlConnection) -> Result<Vec<T>, Error>;
    fn get_by_id(connection: &mut MysqlConnection, id: i32) -> Result<Vec<T>, Error>;
}
pub trait GetTools<T, U>: Get<T> {
    fn get_by_info(connection: &mut MysqlConnection, data: U) -> Result<T, Error>;
}

pub trait Post<T> {
    fn create(connection: &mut MysqlConnection, item: T) -> Result<usize, Error>;
}

pub trait PostWithExtra<T> {
    fn create(connection: &mut MysqlConnection, item: T, extra: Vec<i32>) -> Result<usize, Error>;
}

pub trait Patch<T> {
    fn edit(connection: &mut MysqlConnection, item: T) -> Result<usize, Error>;
}

pub trait Delete<T> {
    fn delete(connection: &mut MysqlConnection, item: T) -> Result<usize, Error>;
}
pub trait DeleteTools<T>: Delete<T> {
    fn delete_by_id(connection: &mut MysqlConnection, id: i32) -> Result<usize, Error>;
}
