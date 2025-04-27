#[macro_use]
extern crate rocket;
use api::post_handler;
use infrastructure::establish_connection_pool;
use rocket::State;

#[launch]
fn rocket() -> _ {
    let pool = establish_connection_pool();
    rocket::build()
        .manage(pool)
        .mount(
            "/api",
            routes![
                post_handler::create_user_handler,
                post_handler::create_post_handler,
                post_handler::list_posts_handler,
            ],
        )
}