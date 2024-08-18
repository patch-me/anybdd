use crate::models::NewTag;
use crate::tag::TagService;
use crate::Post;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use diesel::{insert_into, prelude::*};

impl Post<NewTag> for TagService {
    fn create(connection: &mut MysqlConnection, item: NewTag) -> Result<usize, Error> {
        use crate::schema::tags;
        insert_into(tags::table).values(item).execute(connection)
    }
}
