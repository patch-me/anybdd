use crate::models::Tag;
use crate::tag::TagService;
use crate::Patch;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{prelude::*, update};

impl Patch<Tag> for TagService {
    fn edit(connection: &mut MysqlConnection, item: Tag) -> Result<usize, Error> {
        use crate::schema::tags;
        update(tags::table.find(item.id))
            .set(item)
            .execute(connection)
    }
}
