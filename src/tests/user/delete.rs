use std::sync::Arc;

use diesel::{dsl::count, prelude::*};

// Import necessary modules and functions
use crate::connection::{get_connection_pool, get_connection_url};
use crate::{
  models::user::{NewUser, User},
  schema::users::dsl::*,
};

#[test]
fn test_delete_by_name() {
  let connection =
    &mut MysqlConnection::establish(&get_connection_url()).unwrap();
  diesel::delete(users).execute(connection).unwrap();
  let user = User::create(connection, &NewUser {
    username: String::from("john_doe"),
    password: String::from("secure_password"),
  })
  .unwrap();
  let mut count_user: i64 =
    users.select(count(username)).first::<i64>(connection).unwrap();

  assert_eq!(count_user, 1);
  User::delete(connection, user.id).unwrap();
  count_user = users.select(count(username)).first::<i64>(connection).unwrap();

  assert_eq!(count_user, 0);
}
