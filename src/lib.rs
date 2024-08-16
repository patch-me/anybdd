pub mod connection;
pub mod models;
pub mod schema;
pub mod user;
use std::usize;

use diesel::mysql::MysqlConnection;
use diesel::result::Error;

pub trait Get<T> {
    fn get(connection: &mut MysqlConnection) -> Result<Vec<T>, Error>;
    fn get_by_id(connection: &mut MysqlConnection, id: i32) -> Result<Vec<T>, Error>;
}

pub trait Post<T, U> {
    fn create(connection: &mut MysqlConnection, item: U) -> Result<usize, diesel::result::Error>;
}

pub trait Patch<T> {
    fn edit(connection: &mut MysqlConnection, item: T) -> Result<usize, Error>;
}

pub trait Delete<T> {
    fn delete(connection: &mut MysqlConnection, item: T) -> Result<usize, Error>;
    fn delete_by_id(connection: &mut MysqlConnection, id: i32) -> Result<usize, Error>;
}
