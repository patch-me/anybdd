use crate::models::user::User;
use crate::{Read, ReadResult};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

impl Read for User {
    fn get(connection: &mut MysqlConnection, value: &i32) -> ReadResult<Self, Error> {
        // Implement logic to get a single user here
        use crate::schema::users;
        match users::table
            .filter(users::id.eq(value))
            .first::<User>(connection)
        {
            Ok(user) => ReadResult::Single(user), // Success case
            Err(err) => ReadResult::Error(err),   // Error case
        }
    }

    fn get_by_name(connection: &mut MysqlConnection, value: &String) -> ReadResult<User, Error> {
        // Implement logic to get all users by name here
        use crate::schema::users;
        match users::table
            .filter(users::username.eq(value))
            .load::<User>(connection)
        {
            Ok(users) => ReadResult::Multiple(users), // Success case with multiple users
            Err(err) => ReadResult::Error(err),       // Error case
        }
    }
}
