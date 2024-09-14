use diesel::{mysql::MysqlConnection, prelude::*, result::Error};

use crate::models::user::User;

impl User {
  pub fn list(connection: &mut MysqlConnection) -> Result<Vec<Self>, Error> {
    use crate::schema::users::dsl::*;
    users.load(connection)
  }

  pub fn count(connection: &mut MysqlConnection) -> Result<i64, Error> {
    use diesel::dsl::count;

    use crate::schema::users::dsl::*;
    users.select(count(username)).first::<i64>(connection)
  }
}
