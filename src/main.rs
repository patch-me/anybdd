use anybdd::connection::get_connection_pool;
use std::error::Error;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    let pool = get_connection_pool();
    let mut threads = vec![];
    let max_users_to_create = 1;

    for i in 0..max_users_to_create {
        let pool = pool.clone();
        threads.push(thread::spawn({ move || {} }))
    }
    Ok(())
}
