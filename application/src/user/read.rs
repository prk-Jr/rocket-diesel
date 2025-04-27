use infrastructure::establish_connection;
use diesel::prelude::*;

pub fn get_user_by_username(name: &str) -> Result<domain::models::User, Box<dyn std::error::Error>> {
    use domain::schema::users::dsl::*;
    let mut conn = establish_connection();
    let user = users.filter(username.eq(name)).first::<domain::models::User>(&mut conn)?;
    Ok(user)
}
pub fn get_user_by_id(user_id: i32) -> Result<domain::models::User, Box<dyn std::error::Error>> {
    use domain::schema::users::dsl::*;
    let mut conn = establish_connection();
    let user = users.filter(id.eq(user_id)).first::<domain::models::User>(&mut conn)?;
    Ok(user)
}