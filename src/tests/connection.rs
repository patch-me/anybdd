use crate::connection::{get_connection_pool, get_connection_url};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::Pool; // Import prelude to use Diesel's traits

#[test]
fn test_connection_url() {
    let url = get_connection_url();
    assert!(
        url.contains("mysql") || url.contains("postgres"),
        "The URL does not contain 'mysql' or 'postgres'"
    );
}

#[test]
fn test_pool_connection() {
    // Ensure DATABASE_URL is set correctly in your environment
    let pool: Pool<diesel::r2d2::ConnectionManager<MysqlConnection>> = get_connection_pool();
    // Try to get a connection from the pool
    let mut conn = pool
        .get()
        .expect("Failed to get a connection from the pool");
    // Perform a simple query to check if the connection is working
    let result = diesel::sql_query("SELECT 1").execute(&mut conn);
    assert!(
        result.is_ok(),
        "The connection from the pool is not valid or the query failed"
    );
}
