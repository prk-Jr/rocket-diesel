use domain::models::{Post, User};
use shared::response_models::PostResponse;
use infrastructure::establish_connection;
use diesel::prelude::*;
use diesel::sql_types::{Array, Nullable, Text};
use diesel::dsl::sql;
// use diesel::expression::dsl::sql;
// use diesel::expression::dsl::sql;

pub fn list_posts(page: i64, limit: i64, search: Option<String>) -> Result<(Vec<PostResponse>, i64), Box<dyn std::error::Error>> {
    use domain::schema::{posts, users, posts_tags};

    let mut conn = establish_connection();

    // Count total posts
    let mut count_query = posts::table.into_boxed();
    if let Some(search_text) = search {
        let pattern = format!("%{}%", search_text);
        count_query = count_query.filter(posts::title.ilike(&pattern).or(posts::body.ilike(&pattern)));
    }
    let total_count = count_query.count().get_result(&mut conn)?;

    // Subquery for tags
    let tags_subquery = posts_tags::table
        .group_by(posts_tags::post_id)
        .select((
            posts_tags::post_id,
            sql::<Array<Nullable<Text>>>("coalesce(array_agg(posts_tags.tag), '{}')").nullable(),
        ))
        .into_boxed();

    // Main query
    let mut query = posts::table
        .left_join(users::table.on(posts::created_by.eq(users::id.nullable())))
        .left_join(tags_subquery.on(posts::id.eq(posts_tags::post_id)))
        .select((
            posts::all_columns,
            users::all_columns.nullable(),
            sql::<Array<Nullable<Text>>>("coalesce(tags_subquery.array_agg, '{}')").nullable(),
        ))
        .into_boxed();

    if let Some(search_text) = search {
        let pattern = format!("%{}%", search_text);
        query = query.filter(posts::title.ilike(&pattern).or(posts::body.ilike(&pattern)));
    }

    let posts_with_data = query
        .order(posts::id.desc())
        .limit(limit)
        .offset((page - 1) * limit)
        .load::<(Post, Option<User>, Option<Vec<String>>)>(&mut conn)?;

    let post_responses = posts_with_data.into_iter().map(|(post, user, tags)| {
        PostResponse {
            id: post.id,
            created_by: user.map(|u| shared::response_models::UserResponse {
                user_id: u.id,
                username: u.username,
                first_name: u.first_name,
                last_name: u.last_name,
            }),
            title: post.title,
            body: post.body,
            tags: tags.unwrap_or_else(Vec::new),
        }
    }).collect();

    Ok((post_responses, total_count))
}