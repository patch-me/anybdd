use crate::models::User;
use crate::user::UserService;
use crate::{Delete, DeleteTools};
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{delete, prelude::*};

impl Delete<User> for UserService {
    fn delete(connection: &mut MysqlConnection, item: User) -> Result<usize, Error> {
        use crate::schema::users;
        delete(users::table.find(item.id)).execute(connection)
    }
}

impl DeleteTools<User> for UserService {
    fn delete_by_id(connection: &mut MysqlConnection, id: i32) -> Result<usize, Error> {
        use crate::schema::users;
        delete(users::table)
            .filter(users::id.eq(id))
            .execute(connection)
    }
}
