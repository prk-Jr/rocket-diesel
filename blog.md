//file:  Cargo.toml 
```toml
[workspace]
members = [
  "api",
  "domain",
  "infrastructure",
  "application",
  "shared",
]
```

//file:  api/Cargo.toml 
```toml
[package]
name = "api"
version = "0.1.0"
edition = "2021"

[dependencies]
domain = { path = "../domain" }
application = { path = "../application" }
shared = { path = "../shared" }
infrastructure = { path = "../infrastructure" }
rocket = { version = "0.5.0-rc.2", features = ["json"] }
serde_json = "1.0.88"
```

//file:  api/src/bin/main.rs 
```rust
#[macro_use] extern crate rocket;
use api::post_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            post_handler::create_user_handler,
            post_handler::create_post_handler,
            post_handler::list_posts_handler,
        ])
}
```

//file:  api/src/lib.rs 
```rust
pub mod post_handler;
```

//file:  api/src/post_handler.rs 
```rust
use rocket::{get, post};
use rocket::serde::json::Json;
use rocket::http::Status;
use shared::response_models::{Response, ResponseBody, PostResponse, PaginatedResponse};
use domain::models::{NewUser, CreatePostRequest};
use application::user::create;
use application::post::{create, read};
use infrastructure::establish_connection;

#[post("/users", format = "json", data = "<new_user>")]
pub fn create_user_handler(new_user: Json<NewUser>) -> Result<Json<Response>, Status> {
    let user = create::create_user(new_user.into_inner()).map_err(|_| Status::InternalServerError)?;
    let response = Response { body: ResponseBody::User(user) };
    Ok(Json(response))
}

#[post("/posts", format = "json", data = "<create_post_request>")]
pub fn create_post_handler(create_post_request: Json<CreatePostRequest>) -> Result<Json<Response>, Status> {
    let post_response = create::create_post(create_post_request.into_inner()).map_err(|_| Status::InternalServerError)?;
    let response = Response { body: ResponseBody::Post(post_response) };
    Ok(Json(response))
}

#[get("/posts?<page>&<limit>&<search>")]
pub fn list_posts_handler(page: Option<i64>, limit: Option<i64>, search: Option<String>) -> Result<Json<Response>, Status> {
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10);
    let (posts, total_count) = read::list_posts(page, limit, search).map_err(|_| Status::InternalServerError)?;
    let total_pages = (total_count + limit - 1) / limit;
    let from = (page - 1) * limit + 1;
    let to = from + posts.len() as i64 - 1;
    let paginated_response = PaginatedResponse {
        records: posts,
        meta: shared::response_models::Meta {
            current_page: page,
            per_page: limit,
            from,
            to,
            total_pages,
            total_docs: total_count,
        },
    };
    let response = Response { body: ResponseBody::Posts(paginated_response) };
    Ok(Json(response))
}
```

//file:  application/Cargo.toml 
```toml
[package]
name = "application"
version = "0.1.0"
edition = "2021"

[dependencies]
domain = { path = "../domain" }
infrastructure = { path = "../infrastructure" }
shared = { path = "../shared" }
diesel = { version = "2.0.0", features = ["postgres"] }
serde_json = "1.0.88"
rocket = { version = "0.5.0-rc.2", features = ["json"] }
```

//file:  application/src/lib.rs 
```rust
pub mod user;
pub mod post;
```

//file:  application/src/user/create.rs 
```rust
use domain::models::{User, NewUser};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;

pub fn create_user(new_user: NewUser) -> Result<User, Box<dyn std::error::Error>> {
    use domain::schema::users;
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&mut establish_connection())
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
```

//file:  application/src/user/mod.rs 
```rust
pub mod create;
```

//file:  application/src/post/create.rs 
```rust
use domain::models::{NewPost, CreatePostRequest, NewPostTag};
use shared::response_models::PostResponse;
use infrastructure::establish_connection;
use diesel::prelude::*;

pub fn create_post(request: CreatePostRequest) -> Result<PostResponse, Box<dyn std::error::Error>> {
    use domain::schema::{posts, posts_tags, users};
    let mut conn = establish_connection();
    let new_post = NewPost {
        created_by: request.created_by,
        title: request.title,
        body: request.body,
    };
    let post = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result::<domain::models::Post>(&mut conn)?;
    for tag in request.tags {
        let new_tag = NewPostTag { post_id: post.id, tag };
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
```

//file:  application/src/post/read.rs 
```rust
use domain::models::{Post, User};
use shared::response_models::PostResponse;
use infrastructure::establish_connection;
use diesel::prelude::*;
use diesel::sql_types::{Array, Nullable, Text};
use diesel::expression::dsl::sql;

pub fn list_posts(page: i64, limit: i64, search: Option<String>) -> Result<(Vec<PostResponse>, i64), Box<dyn std::error::Error>> {
    use domain::schema::{posts, users, posts_tags};
    let mut conn = establish_connection();
    let mut query = posts::table.into_boxed();
    if let Some(search_text) = search {
        let pattern = format!("%{}%", search_text);
        query = query.filter(posts::title.ilike(&pattern).or(posts::body.ilike(&pattern)));
    }
    let total_count = query.count().get_result(&mut conn)?;
    let posts_with_data = query
        .left_join(users::table.on(posts::created_by.eq(users::id.nullable())))
        .left_join(posts_tags::table.on(posts::id.eq(posts_tags::post_id)))
        .select((
            posts::all_columns,
            users::all_columns.nullable(),
            sql::<Array<Nullable<Text>>>("array_agg(posts_tags.tag)").nullable(),
        ))
        .group_by(posts::id)
        .group_by(users::id)
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
```

//file:  application/src/post/mod.rs 
```rust
pub mod create;
pub mod read;
```

//file:  domain/Cargo.toml 
```toml
[package]
name = "domain"
version = "0.1.0"
edition = "2021"

[dependencies]
rocket = { version = "0.5.0-rc.2", features = ["json"] }
diesel = { version = "2.0.0", features = ["postgres"] }
serde = { version = "1.0.147", features = ["derive"] }
```

//file:  domain/src/lib.rs 
```rust
pub mod models;
pub mod schema;
```

//file:  domain/src/models.rs 
```rust
use crate::schema::{users, posts, posts_tags};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable, Serialize, Identifiable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub created_by: Option<i32>,
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub created_by: Option<i32>,
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[diesel(table_name = posts_tags)]
pub struct NewPostTag {
    pub post_id: i32,
    pub tag: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreatePostRequest {
    pub created_by: Option<i32>,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
}
```

//file:  domain/src/schema.rs 
```rust
diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        created_by -> Nullable<Int4>,
        title -> Varchar,
        body -> Text,
    }
}

diesel::table! {
    posts_tags (post_id, tag) {
        post_id -> Int4,
        tag -> Varchar,
    }
}

diesel::joinable!(posts -> users (created_by));
diesel::joinable!(posts_tags -> posts (post_id));
diesel::allow_tables_to_appear_in_same_query!(users, posts, posts_tags);
```

//file:  infrastructure/Cargo.toml 
```toml
[package]
name = "infrastructure"
version = "0.1.0"
edition = "2021"

[dependencies]
diesel = { version = "2.0.0", features = ["postgres"] }
dotenvy = "0.15"
```

//file:  infrastructure/src/lib.rs 
```rust
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
```

//file:  shared/Cargo.toml 
```toml
[package]
name = "shared"
version = "0.1.0"
edition = "2021"

[dependencies]
domain = { path = "../domain" }
rocket = { version = "0.5.0-rc.2", features = ["json"] }
serde = { version = "1.0.147", features = ["derive"] }
```

//file:  shared/src/lib.rs 
```rust
pub mod response_models;
```

//file:  shared/src/response_models.rs 
```rust
use domain::models::{Post, User};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserResponse {
    pub user_id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PostResponse {
    pub id: i32,
    pub created_by: Option<UserResponse>,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PaginatedResponse<T> {
    pub records: Vec<T>,
    pub meta: Meta,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Meta {
    pub current_page: i64,
    pub per_page: i64,
    pub from: i64,
    pub to: i64,
    pub total_pages: i64,
    pub total_docs: i64,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum ResponseBody {
    Message(String),
    User(User),
    Post(PostResponse),
    Posts(PaginatedResponse<PostResponse>),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
```
