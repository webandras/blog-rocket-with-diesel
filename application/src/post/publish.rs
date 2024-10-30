use domain::models::{Post, PostWithRelations};
use shared::response_models::{Response, ResponseBody};
use rocket::response::status::NotFound;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rocket::State;
use infrastructure::db_pool::ServerState;
use crate::post;

pub fn publish_post(state: &State<ServerState>, post_id: i32) -> Result<PostWithRelations, NotFound<String>> {
    use domain::schema::posts::dsl::*;

    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = state.db_pool.get().expect("Could not connect to DB");

    match diesel::update(posts.find(post_id)).set(
        published.eq(true)
    ).get_result::<Post>(&mut conn) {
        Ok(post) => {
            let post: PostWithRelations = post::read::list_post(&state, post.id)?;
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