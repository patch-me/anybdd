use diesel::dsl::count_star;
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::connection::get_connection_pool;
use crate::models::user::{NewUser, User};
use crate::schema::users;
use std::sync::Arc;

#[test]
fn test_delete_by_name() {
    // let user_name: String = "updated_user".to_string();
    // let pass_word: String = "updated_password".to_string();
    // // Set up connection pool for the database (using Arc for thread safety)
    // let pool = Arc::new(get_connection_pool());
    //
    // // Get a connection from the pool
    // let mut connection = pool
    //     .get()
    //     .expect("Failed to get a connection from the pool.");
    // match delete_query.execute(&mut connection) {
    //     Ok(rows) => {
    //         println!("Successfully deleted {} row(s).", rows);
    //         // assert!(
    //         //     users::table
    //         //         .filter(users::dsl::username.eq(username))
    //         //         .count() // Count is an aggregation, it should not be treated as an iterator
    //         //         .get_result(connection)
    //         //         == 0
    //         // );
    //         // Count the number of users with the given username
    //         let user: Option<users::table> = users::table
    //             .filter(username.eq(&username))
    //             .first(conn)
    //             .optional()
    //             .unwrap_or_else(|_| panic!("Error loading user"));
    //
    //         assert!(user.is_none());
    //     }
    //     Err(err) => println!("Error deleting user: {:?}", err),
    // }
}
