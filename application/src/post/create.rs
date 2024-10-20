use domain::models::{CreatePost, Post, PostWithRelations};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use crate::post;

pub fn create_post(post: Json<CreatePost>) -> Created<String> {
    use domain::schema::posts;

    let post = post.into_inner();

    match diesel::insert_into(posts::table).values(&post).get_result::<Post>(&mut establish_connection()) {
        Ok(post) => {
            let post: PostWithRelations = post::read::list_post(post.id).unwrap();
            let response = Response { body: ResponseBody::PostWithRelations(post) };

            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}