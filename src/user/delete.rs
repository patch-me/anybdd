use diesel::{mysql::MysqlConnection, prelude::*, result::Error};

use crate::models::user::User;

impl User {
  pub fn delete(
    connection: &mut MysqlConnection,
    value: i32,
  ) -> Result<bool, Error> {
    use crate::schema::users::dsl::*;

    // Perform the delete operation
    match diesel::delete(users.filter(id.eq(value))).execute(connection) {
      Ok(rows_affected) => Ok(rows_affected > 0), /* Return true if at least one row was affected */
      Err(e) => Err(e), // Return the Diesel error directly
    }
  }
}
