use diesel::{mysql::MysqlConnection, prelude::*, result::Error};

use crate::models::user::User;

impl User {
  pub fn check_list_user(
    connection: &mut MysqlConnection,
    value: &Vec<&Self>,
  ) -> Result<bool, Error> {
    let all_users: Vec<(i32, String)> = User::list_name_and_id_map(connection)?;
    let value_users: Vec<(i32, String)> = value.iter() // Iterate over the Vec<User>
        .map(|user| (user.id, user.username.clone())) // Map each User into (id, username)
        .collect();
    Ok(all_users.iter().all(|item| value_users.contains(item)))
  }
}
