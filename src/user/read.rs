use diesel::{mysql::MysqlConnection, prelude::*, result::Error};

use crate::models::user::User;

impl User {
  pub fn get(
    connection: &mut MysqlConnection,
    value: &i32,
  ) -> Result<Self, Error> {
    // Implement logic to get a single user here
    use crate::schema::users;
    users::table.filter(users::id.eq(value)).first::<User>(connection)
  }

  pub fn get_by_name(
    connection: &mut MysqlConnection,
    value: &String,
  ) -> Result<Self, Error> {
    // Implement logic to get all users by name here
    use crate::schema::users;
    users::table.filter(users::username.eq(value)).first::<User>(connection)
  }
}
