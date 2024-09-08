use crate::models::user::User;
use crate::Delete;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

impl Delete for User {
    fn delete(connection: &mut MysqlConnection, value: &i32) -> Result<bool, Error> {
        use crate::schema::users;

        match diesel::delete(users::table)
            .filter(users::id.eq(value))
            .execute(connection)
        {
            Ok(rows_affected) => Ok(rows_affected > 0), // Return true if at least one row was affected
            Err(e) => Err(e.into()),                    // Convert Diesel's error to your Error type
        }
    }
}
