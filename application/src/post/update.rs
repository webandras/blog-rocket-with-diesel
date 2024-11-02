use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use diesel::expression_methods::ExpressionMethods;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::State;
use domain::models::{Post, PostWithRelations};
use infrastructure::db_pool::ServerState;
use shared::response_models::{Response, ResponseBody};
use crate::post;

pub fn update_post(state: &State<ServerState>, post_id: i32, post: Json<Post>) -> Result<PostWithRelations, NotFound<String>> {
    use domain::schema::posts::dsl::*;

    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = state.db_pool.get().expect("Could not connect to DB");

    match diesel::update(posts.find(post_id)).set(
        (
            title.eq(&post.title),
            body.eq(&post.body),
            genre.eq(&post.genre),
            published.eq(&post.published),
            author_id.eq(&post.author_id.unwrap_or_default())
        )
    ).get_result::<Post>(&mut conn) {
        Ok(post) => {
            let post: PostWithRelations = post::read::list_post(state, post.id)?;
            Ok(post)

        },
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { data: ResponseBody::Error(format!("Error updating post with id {} - {}", post_id, err)) };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}