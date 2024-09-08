use crate::models::user::User;
use crate::Update;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

impl Update for User {
    fn update(connection: &mut MysqlConnection, value: &Self) -> Result<bool, Error> {
        use crate::schema::users::dsl::*;

        match diesel::update(users).set(value).execute(connection) {
            Ok(rows_affected) => Ok(rows_affected > 0), // Return true if at least one row was affected
            Err(e) => Err(e.into()),                    // Convert Diesel's error to your Error type
        }
    }
}
