use domain::models::{Post, User};
use shared::response_models::PostResponse;
use infrastructure::DbConnection;
use diesel::prelude::*;
use diesel::sql_types::{Array, Text};
use diesel::dsl::sql;
use std::collections::HashMap;

pub fn list_posts(conn: &mut DbConnection, page: i64, limit: i64, search: Option<String>) -> Result<(Vec<PostResponse>, i64), Box<dyn std::error::Error>> {
    use domain::schema::{posts, users, posts_tags};

    // Count total posts
    let mut count_query = posts::table.into_boxed();
    if let Some(search_text) = &search {
        let pattern = format!("%{}%", search_text);
        count_query = count_query.filter(posts::title.ilike(pattern.clone()).or(posts::body.ilike(pattern)));
    }
    let total_count = count_query.count().get_result(conn)?;

    // Query posts
    let mut posts_query = posts::table.into_boxed();
    if let Some(search_text) = &search {
        let pattern = format!("%{}%", search_text);
        posts_query = posts_query.filter(posts::title.ilike(pattern.clone()).or(posts::body.ilike(pattern)));
    }
    let posts = posts_query
        .order(posts::id.desc())
        .limit(limit)
        .offset((page - 1) * limit)
        .load::<Post>(conn)?;

    // Get unique user IDs and post IDs
    let user_ids: Vec<i32> = posts.iter().filter_map(|post| post.created_by).collect::<Vec<_>>();
    let post_ids: Vec<i32> = posts.iter().map(|post| post.id).collect();

    // Query users
    let users = users::table
        .filter(users::id.eq_any(user_ids))
        .load::<User>(conn)?;
    let user_map: HashMap<i32, User> = users.into_iter().map(|user| (user.id, user)).collect();

    // Query tags
    let tags = posts_tags::table
        .filter(posts_tags::post_id.eq_any(post_ids))
        .group_by(posts_tags::post_id)
        .select((
            posts_tags::post_id,
            sql::<Array<Text>>("COALESCE(ARRAY_AGG(posts_tags.tag), '{}')").nullable(),
        ))
        .load::<(i32, Option<Vec<String>>)>(conn)?;
    let tags_map: HashMap<i32, Vec<String>> = tags
        .into_iter()
        .map(|(post_id, tags)| (post_id, tags.unwrap_or_default()))
        .collect();

    // Build responses
    let post_responses = posts.into_iter().map(|post| {
        PostResponse {
            id: post.id,
            created_by: post.created_by.and_then(|user_id| user_map.get(&user_id).map(|user| shared::response_models::UserResponse {
                user_id: user.id,
                username: user.username.clone(),
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
            })),
            title: post.title,
            body: post.body,
            tags: tags_map.get(&post.id).cloned().unwrap_or_default(),
        }
    }).collect();

    Ok((post_responses, total_count))
}