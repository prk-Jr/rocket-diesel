use infrastructure::DbConnection;
use diesel::prelude::*;

pub fn get_user_by_username(conn: &mut DbConnection, name: &str) -> Result<domain::models::User, Box<dyn std::error::Error>> {
    use domain::schema::users::dsl::*;
    let user = users.filter(username.eq(name)).first::<domain::models::User>(conn)?;
    Ok(user)
}

pub fn get_user_by_id(conn: &mut DbConnection, user_id: i32) -> Result<domain::models::User, Box<dyn std::error::Error>> {
    use domain::schema::users::dsl::*;
    let user = users.filter(id.eq(user_id)).first::<domain::models::User>(conn)?;
    Ok(user)
}