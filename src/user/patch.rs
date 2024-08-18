use crate::models::User;
use crate::user::UserService;
use crate::Patch;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{prelude::*, update};

impl Patch<User> for UserService {
    fn edit(connection: &mut MysqlConnection, item: User) -> Result<usize, Error> {
        use crate::schema::users;
        update(users::table.find(item.id))
            .set(item)
            .execute(connection)
    }
}
