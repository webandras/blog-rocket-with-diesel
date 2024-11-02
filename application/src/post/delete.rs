use shared::response_models::{Response, ResponseBody};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rocket::response::status::NotFound;
use rocket::State;
use domain::models::{Author, Post, PostWithRelations};
use domain::schema::authors;
use infrastructure::db_pool::ServerState;

pub fn delete_post(state: &State<ServerState>, post_id: i32) -> Result<Vec<PostWithRelations>, NotFound<String>> {
    use domain::schema::posts::dsl::*;
    use domain::schema::posts;

    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = state.db_pool.get().expect("Could not connect to DB");
    let response: Response;

    let num_deleted = match diesel::delete(posts.filter(id.eq(post_id))).execute(&mut conn) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { data: ResponseBody::Error(format!("Error deleting post with id {} - {}", post_id, err)) };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        }
    };

    if num_deleted > 0 {
        match posts::table
            .left_join(authors::table)
            .select((Post::as_select(), Option::<Author>::as_select()))
            .load::<(Post, Option<Author>)>(&mut conn) {
            Ok(result) => {
                let mut _posts: Vec<PostWithRelations> = result
                    .into_iter()
                    .map(|(post, author)| PostWithRelations { post, author })
                    .collect();

                _posts.sort();
                Ok(_posts)
            }
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        response = Response { data: ResponseBody::Error(format!("Error - no post with id {}", post_id)) };
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    }
}