use crate::models::{NewUser, User};
use crate::schema::users;
use crate::{Delete, Get, Patch, Post};
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{delete, insert_into, prelude::*, update};

pub struct UserService;

impl Get<User> for UserService {
    fn get(connection: &mut MysqlConnection) -> Result<Vec<User>, Error> {
        // Implement logic to get all users here
        use crate::schema::users::dsl::*;
        users.load::<User>(connection)
    }

    fn get_by_id(connection: &mut MysqlConnection, id_user: i32) -> Result<Vec<User>, Error> {
        // Implement logic to get a user by id here
        use crate::schema::users;
        users::table
            .filter(crate::schema::users::id.eq(id_user))
            .load::<User>(connection)
    }
}
impl Post<User, NewUser> for UserService {
    fn create(connection: &mut MysqlConnection, item: NewUser) -> Result<usize, Error> {
        use crate::schema::users;
        insert_into(users::table).values(item).execute(connection)
    }
}
impl Patch<User> for UserService {
    fn edit(connection: &mut MysqlConnection, item: User) -> Result<usize, Error> {
        use crate::schema::users;
        update(users::table.find(item.id))
            .set(item)
            .execute(connection)
    }
}
impl Delete<User> for UserService {
    fn delete(connection: &mut MysqlConnection, item: User) -> Result<usize, Error> {
        delete(users::table.find(item.id)).execute(connection)
    }
    fn delete_by_id(connection: &mut MysqlConnection, id: i32) -> Result<usize, Error> {
        delete(users::table)
            .filter(users::id.eq(id))
            .execute(connection)
    }
}
