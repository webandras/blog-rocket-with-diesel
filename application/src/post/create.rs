use domain::models::{Post, NewPost};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_post(post: Json<NewPost>) -> Created<String> {
    use domain::schema::posts;

    // let now = Utc::now().naive_utc();
    let post = post.into_inner();
    // post.created_at = None;
    // post.updated_at = None;

    match diesel::insert_into(posts::table).values(&post).get_result::<Post>(&mut establish_connection()) {
        Ok(post) => {
            let response = Response { body: ResponseBody::Post(post) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}