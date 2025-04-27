use domain::models::{User, NewUser};
use infrastructure::DbConnection;
use diesel::prelude::*;

pub fn create_user(conn: &mut DbConnection, new_user: NewUser) -> Result<User, Box<dyn std::error::Error>> {
    use domain::schema::users;
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}