use std::sync::Arc;

use diesel::{dsl::count, prelude::*};

// Import necessary modules and functions
use crate::connection::get_connection_pool;
use crate::{models::user::NewUser, schema::users::dsl::*};

#[test]
fn test_create_user() {
  // Set up connection pool for the database (using Arc for thread safety)
  let pool = Arc::new(get_connection_pool());

  // Get a connection from the pool
  let mut connection =
    pool.get().expect("Failed to get a connection from the pool.");

  diesel::delete(users).execute(&mut connection).unwrap();
  // // Create a new user object
  NewUser::create(&mut connection, &NewUser {
    username: String::from("john_doe"),
    password: String::from("secure_password"),
  })
  .unwrap();

  let count: i64 =
    users.select(count(username)).first::<i64>(&mut connection).unwrap();

  assert_eq!(count, 1);

  diesel::delete(users).execute(&mut connection).unwrap();
}
