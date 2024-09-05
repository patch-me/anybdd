use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use std::env;

pub fn get_connection_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    let manager = ConnectionManager::<MysqlConnection>::new(get_connection_url());
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
pub fn get_connection_url() -> String {
    dotenv().ok();
    return env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}
