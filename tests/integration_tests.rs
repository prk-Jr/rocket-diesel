#[cfg(test)]
mod tests {
    use diesel::prelude::*;
    use diesel::sqlite::SqliteConnection;
    use crate::application::user::create as user_create;
    use crate::application::post::{create as post_create, read};
    use crate::domain::models::{NewUser, CreatePostRequest};
    use crate::shared::response_models::PostResponse;

    fn setup_db() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        diesel::sql_query("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL UNIQUE, first_name TEXT NOT NULL, last_name TEXT NOT NULL)").execute(&mut conn).unwrap();
        diesel::sql_query("CREATE TABLE posts (id INTEGER PRIMARY KEY AUTOINCREMENT, created_by INTEGER, title TEXT NOT NULL, body TEXT NOT NULL, FOREIGN KEY(created_by) REFERENCES users(id))").execute(&mut conn).unwrap();
        diesel::sql_query("CREATE TABLE posts_tags (post_id INTEGER NOT NULL, tag TEXT NOT NULL, PRIMARY KEY (post_id, tag), FOREIGN KEY(post_id) REFERENCES posts(id))").execute(&mut conn).unwrap();
        conn
    }

    #[test]
    fn test_create_user() {
        let mut conn = setup_db();
        let new_user = NewUser {
            username: "testuser".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
        };
        let user = user_create::create_user(new_user).unwrap();
        assert_eq!(user.username, "testuser");
    }

    #[test]
    fn test_create_post_with_tags() {
        let mut conn = setup_db();
        let new_user = NewUser {
            username: "testuser".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
        };
        let user = user_create::create_user(new_user).unwrap();
        let input = CreatePostRequest {
            created_by: Some(user.id),
            title: "Test Post".to_string(),
            body: "This is a test post".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
        };
        let post = post_create::create_post(input).unwrap();
        assert_eq!(post.title, "Test Post");
        assert_eq!(post.tags, vec!["tag1".to_string(), "tag2".to_string()]);
    }

    #[test]
    fn test_list_posts() {
        let mut conn = setup_db();
        let new_user = NewUser {
            username: "testuser".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
        };
        let user = user_create::create_user(new_user).unwrap();
        let input = CreatePostRequest {
            created_by: Some(user.id),
            title: "Test Post".to_string(),
            body: "This is a test post".to_string(),
            tags: vec!["tag1".to_string()],
        };
        post_create::create_post(input).unwrap();
        let (posts, total_count) = read::list_posts(1, 10, None).unwrap();
        assert_eq!(total_count, 1);
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].title, "Test Post");
        assert_eq!(posts[0].tags, vec!["tag1".to_string()]);
    }

    #[test]
    fn test_list_posts_with_search() {
        let mut conn = setup_db();
        let new_user = NewUser {
            username: "testuser".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
        };
        let user = user_create::create_user(new_user).unwrap();
        let input1 = CreatePostRequest {
            created_by: Some(user.id),
            title: "Test Post".to_string(),
            body: "This is a test post".to_string(),
            tags: vec!["tag1".to_string()],
        };
        let input2 = CreatePostRequest {
            created_by: Some(user.id),
            title: "Another Post".to_string(),
            body: "Different content".to_string(),
            tags: vec!["tag2".to_string()],
        };
        post_create::create_post(input1).unwrap();
        post_create::create_post(input2).unwrap();
        let (posts, total_count) = read::list_posts(1, 10, Some("test".to_string())).unwrap();
        assert_eq!(total_count, 1);
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].title, "Test Post");
    }
}