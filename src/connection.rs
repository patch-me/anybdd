use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use std::env;

/// Get connection pool from env file
///
/// ```
/// use anybdd::connection::{get_connection_pool, get_connection_url};
/// use diesel::r2d2::Pool;
/// use diesel::mysql::MysqlConnection;
/// use diesel::prelude::*; // Import prelude to use Diesel's traits
///
/// // Ensure DATABASE_URL is set correctly in your environment
/// let pool: Pool<diesel::r2d2::ConnectionManager<MysqlConnection>> = get_connection_pool();
/// // Try to get a connection from the pool
/// let mut conn = pool.get().expect("Failed to get a connection from the pool");
/// // Perform a simple query to check if the connection is working
/// let result = diesel::sql_query("SELECT 1").execute(& mut conn);
/// assert!(result.is_ok(), "The connection from the pool is not valid or the query failed");
/// ```
///
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
///     url.contains("mysql") || url.contains("postgres"),
///     "The URL does not contain 'mysql' or 'postgres'"
/// );
/// ```
pub fn get_connection_url() -> String {
    dotenv().ok();
    return env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}
