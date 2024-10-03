use diesel::{mysql::MysqlConnection, prelude::*, result::Error};

use crate::models::user::User;

impl User {
  pub fn list(connection: &mut MysqlConnection) -> Result<Vec<Self>, Error> {
    use crate::schema::users::dsl::*;
    users.load(connection)
  }

  pub fn list_name_and_id_map(
    connection: &mut MysqlConnection,
  ) -> Result<Vec<(i32, String)>, Error> {
    use crate::schema::users::dsl::*;
    users
        .select((id, username)) // select id and username
        .load(connection)
  }

  pub fn count(connection: &mut MysqlConnection) -> Result<i64, Error> {
    use diesel::dsl::count;

    use crate::schema::users::dsl::*;
    users.select(count(username)).first::<i64>(connection)
  }
}
