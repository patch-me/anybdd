use crate::models::Tag;
use crate::tag::TagService;
use crate::Get;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

impl Get<Tag> for TagService {
    fn get(connection: &mut MysqlConnection) -> Result<Vec<Tag>, Error> {
        // Implement logic to get all users here
        use crate::schema::tags::dsl::*;
        tags.load::<Tag>(connection)
    }

    fn get_by_id(connection: &mut MysqlConnection, id_user: i32) -> Result<Vec<Tag>, Error> {
        // Implement logic to get a user by id here
        use crate::schema::tags;
        tags::table
            .filter(crate::schema::tags::id.eq(id_user))
            .load::<Tag>(connection)
    }
}
