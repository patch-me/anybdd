use std::env;

use diesel::{
  mysql::MysqlConnection,
  r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;

/// Get connection pool from env file
pub fn get_connection_pool() -> Pool<ConnectionManager<MysqlConnection>> {
  let manager = ConnectionManager::<MysqlConnection>::new(get_connection_url());
  Pool::builder()
    .test_on_check_out(true)
    .build(manager)
    .expect("Could not build connection pool")
}

/// Get URL from env file
///
/// ```
/// use anybdd::connection::get_connection_url;
/// let url = get_connection_url();
/// assert!(
///   url.contains("mysql") || url.contains("postgres"),
///   "The URL does not contain 'mysql' or 'postgres'"
/// );
/// ```
pub fn get_connection_url() -> String {
  dotenv().ok();
  return env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}
