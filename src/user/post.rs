use crate::models::NewUser;
use crate::user::UserService;
use crate::Post;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{insert_into, prelude::*};

impl Post<NewUser> for UserService {
    fn create(connection: &mut MysqlConnection, item: NewUser) -> Result<usize, Error> {
        use crate::schema::users;
        insert_into(users::table).values(item).execute(connection)
    }
}
