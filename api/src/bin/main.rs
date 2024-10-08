#[macro_use]
extern crate rocket;

use api::cors::CORS;
use api::post_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/api", routes![
            post_handler::list_posts_handler,
            post_handler::list_post_handler,
            post_handler::create_post_handler,
            post_handler::publish_post_handler,
            post_handler::update_post_handler,
            post_handler::delete_post_handler,
            post_handler::all_options_handler,
        ])
}