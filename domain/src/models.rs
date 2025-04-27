use crate::schema::{users, posts, posts_tags};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, QueryableByName, Serialize, Identifiable)]
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

#[derive(Queryable, QueryableByName, Serialize, Identifiable)]
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
    pub created_by: i32,
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
    pub created_by: i32,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
}