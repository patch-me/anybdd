use crate::models::Tag;
use crate::tag::TagService;
use crate::{Delete, DeleteTools};
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{delete, prelude::*};

impl Delete<Tag> for TagService {
    fn delete(connection: &mut MysqlConnection, item: Tag) -> Result<usize, Error> {
        use crate::schema::tags;
        delete(tags::table.find(item.id)).execute(connection)
    }
}

impl DeleteTools<Tag> for TagService {
    fn delete_by_id(connection: &mut MysqlConnection, id: i32) -> Result<usize, Error> {
        use crate::schema::tags;
        delete(tags::table)
            .filter(tags::id.eq(id))
            .execute(connection)
    }
}
