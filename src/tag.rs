use crate::models::{NewTag, Tag};
use crate::schema::tags;
use crate::{Delete, Get, Patch, Post};
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{delete, insert_into, prelude::*, update};

pub struct TagService;

impl Get<Tag> for TagService {
    fn get(connection: &mut MysqlConnection) -> Result<Vec<Tag>, Error> {
        // Implement logic to get all tags here
        use crate::schema::tags;
        tags::table.load::<Tag>(connection)
    }

    fn get_by_id(connection: &mut MysqlConnection, id_user: i32) -> Result<Vec<Tag>, Error> {
        // Implement logic to get a user by id here
        use crate::schema::tags;
        tags::table
            .filter(crate::schema::tags::id.eq(id_user))
            .load::<Tag>(connection)
    }
}
impl Post<Tag, NewTag> for TagService {
    fn create(connection: &mut MysqlConnection, item: NewTag) -> Result<usize, Error> {
        use crate::schema::tags;
        insert_into(tags::table).values(item).execute(connection)
    }
}
impl Patch<Tag> for TagService {
    fn edit(connection: &mut MysqlConnection, item: Tag) -> Result<usize, Error> {
        use crate::schema::tags;
        update(tags::table.find(item.id))
            .set(item)
            .execute(connection)
    }
}
impl Delete<Tag> for TagService {
    fn delete(connection: &mut MysqlConnection, item: Tag) -> Result<usize, Error> {
        delete(tags::table.find(item.id)).execute(connection)
    }
    fn delete_by_id(connection: &mut MysqlConnection, id: i32) -> Result<usize, Error> {
        delete(tags::table)
            .filter(tags::id.eq(id))
            .execute(connection)
    }
}
