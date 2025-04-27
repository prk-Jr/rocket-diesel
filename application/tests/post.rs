#[cfg(test)]
mod tests {
    use application::{post::{create::create_post, read::list_posts}, user::create::create_user};
    use domain::models::{NewUser, CreatePostRequest};
    use infrastructure::establish_connection_pool;

    #[test]
    fn test_create_post_with_tags() {
        let pool = establish_connection_pool();
        let mut conn = pool.get().expect("Failed to get connection");

        let new_user = NewUser {
            username: "testuser".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
        };
        let user = create_user(&mut conn, new_user).unwrap();

        let input = CreatePostRequest {
            created_by: user.id,
            title: "Test Post".to_string(),
            body: "This is a test post".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
        };
        let post = create_post(&mut conn, input).unwrap();
        assert_eq!(post.title, "Test Post");
        assert_eq!(post.tags, vec!["tag1".to_string(), "tag2".to_string()]);
    }

    #[test]
    fn test_list_posts() {
        let pool = establish_connection_pool();
        let mut conn = pool.get().expect("Failed to get connection");

        let new_user = NewUser {
            username: "testuser".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
        };
        let user = create_user(&mut conn, new_user).unwrap();

        let input = CreatePostRequest {
            created_by: user.id,
            title: "Test Post".to_string(),
            body: "This is a test post".to_string(),
            tags: vec!["tag1".to_string()],
        };
        create_post(&mut conn, input).unwrap();

        let (posts, total_count) = list_posts(&mut conn, 1, 10, None).unwrap();
        assert_eq!(total_count, 1);
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].title, "Test Post");
        assert_eq!(posts[0].tags, vec!["tag1".to_string()]);
    }

    #[test]
    fn test_list_posts_with_search() {
        let pool = establish_connection_pool();
        let mut conn = pool.get().expect("Failed to get connection");

        let new_user = NewUser {
            username: "testuser".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
        };
        let user = create_user(&mut conn, new_user).unwrap();

        let input1 = CreatePostRequest {
            created_by: user.id,
            title: "Test Post".to_string(),
            body: "This is a test post".to_string(),
            tags: vec!["tag1".to_string()],
        };
        let input2 = CreatePostRequest {
            created_by: user.id,
            title: "Another Post".to_string(),
            body: "Different content".to_string(),
            tags: vec!["tag2".to_string()],
        };
        create_post(&mut conn, input1).unwrap();
        create_post(&mut conn, input2).unwrap();

        let (posts, total_count) = list_posts(&mut conn, 1, 10, Some("test".to_string())).unwrap();
        assert_eq!(total_count, 1);
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].title, "Test Post");
    }
}