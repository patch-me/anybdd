use diesel::{dsl::count, prelude::*};

// Import necessary modules and functions
use crate::connection::get_connection_url;
use crate::{
  models::user::{NewUser, User},
  schema::users::dsl::*,
};

#[test]
fn test_user_creation() {
  // Set up connection pool for the database (using Arc for thread safety)
  // !TODO d ont delete if table is empty
  let connection =
    &mut MysqlConnection::establish(&get_connection_url()).unwrap();

  diesel::delete(users).execute(connection).unwrap();
  // // Create a new user object
  User::create(connection, &NewUser {
    username: String::from("john_doe"),
    password: String::from("secure_password"),
  })
  .unwrap();

  let count: i64 =
    users.select(count(username)).first::<i64>(connection).unwrap();

  assert_eq!(count, 1);
}

#[test]
fn test_user_duplication() {
  // Set up connection pool for the database (using Arc for thread safety)
  let connection =
    &mut MysqlConnection::establish(&get_connection_url()).unwrap();
  diesel::delete(users).execute(connection).unwrap();
  // // Create a new user object
  User::create(connection, &NewUser {
    username: String::from("john_doe"),
    password: String::from("secure_password"),
  })
  .unwrap();
  let result_sql = User::create(connection, &NewUser {
    username: String::from("john_doe"),
    password: String::from("secure_password"),
  });
  // Assert that the result is an error
  assert!(
    result_sql.is_err(),
    "Expected an error, but got success: {:?}",
    result_sql
  );

  let count_user: i64 =
    users.select(count(username)).first::<i64>(connection).unwrap();

  assert_eq!(count_user, 1);
  diesel::delete(users).execute(connection).unwrap();
}
