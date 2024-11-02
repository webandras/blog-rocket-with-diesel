#[macro_use]
extern crate rocket;

use api::cors::CORS;
use api::post_handler;
use api::authors_handler;
use api::general_handler;
use api::general_handler::{bad_gateway, default, forbidden, internal_server_error, not_found, too_many_redirects, unauthorized};
use infrastructure::db_pool::*;

#[launch]
fn rocket() -> _ {
    let db_pool: DbPool = get_connection_pool();
    run_migrations(&db_pool);

    rocket::build()
        .attach(CORS)
        .mount("/api", routes![
            post_handler::list_posts_handler,
            post_handler::list_post_handler,
            post_handler::create_post_handler,
            post_handler::publish_post_handler,
            post_handler::update_post_handler,
            post_handler::delete_post_handler,

            general_handler::all_options_handler,

            authors_handler::list_authors_handler,
            authors_handler::list_author_handler,
            authors_handler::create_author_handler,
            authors_handler::update_author_handler,
            authors_handler::delete_author_handler,
            authors_handler::list_author_posts_handler
        ])
        .manage(ServerState { db_pool })
        .register("/", catchers![
            unauthorized,
            forbidden,
            not_found,
            rocket_validation::validation_catcher,
            too_many_redirects,
            internal_server_error,
            bad_gateway,
            default
        ])

}