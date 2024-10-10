use diesel::{QueryDsl, RunQueryDsl};
use diesel::expression_methods::ExpressionMethods;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use domain::models::{NewPost, Post};
use infrastructure::establish_connection;
use shared::response_models::{Response, ResponseBody};

pub fn update_post(post_id: i32, post: Json<NewPost>) -> Result<Post, NotFound<String>> {
    use domain::schema::posts::dsl::*;

    match diesel::update(posts.find(post_id)).set(
        (
            title.eq(&post.title),
            body.eq(&post.body),
            genre.eq(&post.genre),
            published.eq(&post.published)
        )
    ).get_result::<Post>(&mut establish_connection()) {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error updating post with id {} - {}", post_id, err)) };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}