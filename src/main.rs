use anybdd::connection::get_connection_pool;
use std::error::Error;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    let pool = get_connection_pool();
    // let mut threads = vec![];
    let max_users_to_create = 1;
    let mut connection = pool
        .get()
        .expect("Failed to get a connection from the pool.");

    // for i in 0..max_users_to_create {
    //     let pool = pool.clone();
    //     threads.push(thread::spawn({ move || {} }))
    // }
    use anybdd::schema::users::dsl::*;
    use diesel::dsl::count;
    use diesel::prelude::*;
    let count: i64 = users
        .select(count(username))
        .first::<i64>(&mut connection)
        .unwrap();
    println!("Number of users: {}", count);
    Ok(())
}
