use diesel::{mysql::MysqlConnection, prelude::*, result::Error};

use crate::models::user::{NewUser, User};

impl User {
  pub fn create(
    connection: &mut MysqlConnection,
    value: &NewUser,
  ) -> Result<Self, Error> {
    use crate::schema::users::dsl::*;

    match diesel::insert_into(users).values(value).execute(connection) {
      Ok(_) => users.filter(username.eq(&value.username)).first(connection),
      Err(e) => Err(e.into()), // Convert Diesel's error to your Error type
    }
  }
}
