use crate::models::User;
use crate::user::UserService;
use crate::Get;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

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
