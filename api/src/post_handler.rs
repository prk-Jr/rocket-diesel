use rocket::{get, post, State};
use rocket::serde::json::Json;
use rocket::http::Status;
use shared::response_models::{Response, ResponseBody, PaginatedResponse};
use domain::models::{NewUser, CreatePostRequest};
use application::user;
use application::post;
use infrastructure::DbPool;

#[post("/users", format = "json", data = "<new_user>")]
pub fn create_user_handler(
    pool: &State<DbPool>,
    new_user: Json<NewUser>,
) -> Result<Json<Response>, Status> {
    let mut conn = pool.get().map_err(|_| Status::InternalServerError)?;
    user::read::get_user_by_username(&mut conn, &new_user.username)
        .map_err(|_| Status::AlreadyReported)?;
    let user = user::create::create_user(&mut conn, new_user.into_inner())
        .map_err(|_| Status::InternalServerError)?;
    let response = Response {
        body: ResponseBody::User(user),
    };
    Ok(Json(response))
}

#[post("/posts", format = "json", data = "<create_post_request>")]
pub fn create_post_handler(
    pool: &State<DbPool>,
    create_post_request: Json<CreatePostRequest>,
) -> Result<Json<Response>, Status> {
    let mut conn = pool.get().map_err(|_| Status::InternalServerError)?;
    let post_response = post::create::create_post(&mut conn, create_post_request.into_inner())
        .map_err(|_| Status::InternalServerError)?;
    let response = Response {
        body: ResponseBody::Post(post_response),
    };
    Ok(Json(response))
}

#[get("/posts?<page>&<limit>&<search>")]
pub fn list_posts_handler(
    pool: &State<DbPool>,
    page: Option<i64>,
    limit: Option<i64>,
    search: Option<String>,
) -> Result<Json<Response>, Status> {
    let mut conn = pool.get().map_err(|_| Status::InternalServerError)?;
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10);
    let (posts, total_count) = post::read::list_posts(&mut conn, page, limit, search)
        .map_err(|_| Status::InternalServerError)?;
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
    let response = Response {
        body: ResponseBody::Posts(paginated_response),
    };
    Ok(Json(response))
}