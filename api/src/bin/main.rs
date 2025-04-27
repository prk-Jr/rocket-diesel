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