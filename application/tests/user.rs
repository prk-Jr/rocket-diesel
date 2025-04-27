#[cfg(test)]
mod tests {
    use application::user::{create::create_user, read::get_user_by_username};
    use domain::models::NewUser;
    use infrastructure::establish_connection_pool;

    #[test]
    fn test_create_user() {
        let pool = establish_connection_pool();
        let mut conn = pool.get().expect("Failed to get connection");

        let new_user = NewUser {
            username: "testuser".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
        };
        let user = create_user(&mut conn, new_user).unwrap();
        assert_eq!(user.username, "testuser");
    }

    #[test]
    fn test_get_user_by_username() {
        let pool = establish_connection_pool();
        let mut conn = pool.get().expect("Failed to get connection");

        let new_user = NewUser {
            username: "testuser".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
        };
        create_user(&mut conn, new_user).unwrap();

        let user = get_user_by_username(&mut conn, "testuser").unwrap();
        assert_eq!(user.username, "testuser");
    }
}