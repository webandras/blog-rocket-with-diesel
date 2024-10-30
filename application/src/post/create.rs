use domain::models::{CreatePost, Post, PostWithRelations};
use shared::response_models::{Response, ResponseBody};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::State;
use infrastructure::db_pool::ServerState;
use crate::post;

pub fn create_post(state: &State<ServerState>, post: Json<CreatePost>) -> Created<String> {
    use domain::schema::posts;

    // let mut conn = get_connection_pool().get().unwrap();
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = state.db_pool.get().expect("Could not connect to DB");
    let post = post.into_inner();

    match diesel::insert_into(posts::table).values(&post).get_result::<Post>(&mut conn) {
        Ok(post) => {
            let post: PostWithRelations = post::read::list_post(&state, post.id).unwrap();
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