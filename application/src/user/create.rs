use domain::models::{User, NewUser};
use infrastructure::establish_connection;
use diesel::prelude::*;

pub fn create_user(new_user: NewUser) -> Result<User, Box<dyn std::error::Error>> {
    use domain::schema::users;
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&mut establish_connection())
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}