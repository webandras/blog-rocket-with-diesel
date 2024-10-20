use domain::models::{Post, PostWithRelations};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use diesel::prelude::*;
use crate::post;

pub fn publish_post(post_id: i32) -> Result<PostWithRelations, NotFound<String>> {
    use domain::schema::posts::dsl::*;

    match diesel::update(posts.find(post_id)).set(
        published.eq(true)
    ).get_result::<Post>(&mut establish_connection()) {
        Ok(post) => {
            let post: PostWithRelations = post::read::list_post(post.id)?;
            Ok(post)
        },
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error publishing post with id {} - {}", post_id, err)) };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}