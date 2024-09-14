use diesel::prelude::*;

// Import necessary modules and functions
use crate::connection::get_connection_url;
use crate::{
  models::user::{NewUser, User},
  schema::users::dsl::*,
};

#[test]
fn test_list_user() {
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
  let test_users = User::list(connection).unwrap();
  // check that the first user of test_users is the same as the user created
  assert_eq!(test_users[0].username, "john_doe");
}

#[test]
fn test_user_count() {
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

  let count: i64 = User::count(connection).unwrap();

  assert_eq!(count, 1);
}
