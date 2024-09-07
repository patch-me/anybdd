// Import necessary modules and functions
use crate::connection::get_connection_pool;
use crate::models::user::NewUser;
use crate::Create; // Assuming Create is a trait implemented for NewUser
use std::sync::Arc;

#[test]
fn test_create_user() {
    // Set up connection pool for the database (using Arc for thread safety)
    let pool = Arc::new(get_connection_pool());

    // Get a connection from the pool
    let mut conn = pool
        .get()
        .expect("Failed to get a connection from the pool.");

    // Create a new user object
    let new_user = NewUser {
        username: String::from("john_doe"),
        password: String::from("secure_password"),
    };

    // Attempt to create a new user in the database
    match NewUser::create(&mut conn, new_user) {
        Ok(true) => {
            println!("User created successfully.");
            assert!(true); // Test passes if user creation succeeds
        }
        Ok(false) => {
            println!("User was not created.");
            assert!(false, "User creation returned false unexpectedly.");
        }
        Err(e) => {
            println!("An error occurred: {}", e);
            assert!(false, "User creation failed with error.");
        }
    }
}
