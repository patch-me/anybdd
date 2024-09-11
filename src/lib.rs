#[cfg(test)]
mod tests;

pub mod connection;
pub mod models;
pub mod project;
pub mod schema;
pub mod tag;
pub mod task;
pub mod user;

// use diesel::mysql::MysqlConnection;
// use diesel::result::Error;

// pub trait Read: Sized {
//     fn get(connection: &mut MysqlConnection, value: &i32) -> Result<Vec<Self>, Error>;
// }
// pub trait Create {
//     fn create(connection: &mut MysqlConnection, item: &i32) -> Result<bool, Error>;
// }
//
// pub trait Update {
//     fn update(connection: &mut MysqlConnection, item: &i32) -> Result<bool, Error>;
// }
//
// pub trait Delete {
//     fn delete(connection: &mut MysqlConnection, item: &i32) -> Result<bool, Error>;
// }
// pub trait List {
//     fn list(connection: &mut MysqlConnection, item: &i32) -> Result<bool, Error>;
// }
