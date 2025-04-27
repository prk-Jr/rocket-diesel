use rocket::{get, post};
use rocket::serde::json::Json;
use rocket::http::Status;
use shared::response_models::{Response, ResponseBody,  PaginatedResponse};
use domain::models::{NewUser, CreatePostRequest};
use application::user;
use application::post;

#[post("/users", format = "json", data = "<new_user>")]
pub fn create_user_handler(new_user: Json<NewUser>) -> Result<Json<Response>, Status> {
    let user = user::create::create_user(new_user.into_inner()).map_err(|_| Status::InternalServerError)?;
    let response = Response { body: ResponseBody::User(user) };
    Ok(Json(response))
}

#[post("/posts", format = "json", data = "<create_post_request>")]
pub fn create_post_handler(create_post_request: Json<CreatePostRequest>) -> Result<Json<Response>, Status> {
    let post_response = post::create::create_post(create_post_request.into_inner()).map_err(|_| Status::InternalServerError)?;
    let response = Response { body: ResponseBody::Post(post_response) };
    Ok(Json(response))
}

#[get("/posts?<page>&<limit>&<search>")]
pub fn list_posts_handler(page: Option<i64>, limit: Option<i64>, search: Option<String>) -> Result<Json<Response>, Status> {
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10);
    let (posts, total_count) = post::read::list_posts(page, limit, search).map_err(|_| Status::InternalServerError)?;
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