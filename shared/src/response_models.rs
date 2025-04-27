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