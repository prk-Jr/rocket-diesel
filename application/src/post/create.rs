use domain::models::{NewPost, CreatePostRequest, NewPostTag};
use shared::response_models::PostResponse;
use infrastructure::establish_connection;
use diesel::prelude::*;

pub fn create_post(request: CreatePostRequest) -> Result<PostResponse, Box<dyn std::error::Error>> {
    use domain::schema::{posts, posts_tags, users};
    let mut conn = establish_connection();
    let new_post = NewPost {
        created_by: Some(request.created_by),
        title: request.title,
        body: request.body,
    };
    let post = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result::<domain::models::Post>(&mut conn)?;
    for tag in &request.tags {
        let new_tag = NewPostTag { post_id: post.id, tag: tag.clone() };
        diesel::insert_into(posts_tags::table)
            .values(&new_tag)
            .execute(&mut conn)?;
    }
    let user = if let Some(user_id) = post.created_by {
        users::table.find(user_id).first::<domain::models::User>(&mut conn).ok()
    } else {
        None
    };
    Ok(PostResponse {
        id: post.id,
        created_by: user.map(|u| shared::response_models::UserResponse {
            user_id: u.id,
            username: u.username,
            first_name: u.first_name,
            last_name: u.last_name,
        }),
        title: post.title,
        body: post.body,
        tags: request.tags,
    })
}
