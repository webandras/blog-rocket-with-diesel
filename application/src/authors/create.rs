use domain::models::{Author, CreateAuthor};
use shared::response_models::{Response, ResponseBody};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::State;
use infrastructure::db_pool::ServerState;

pub fn create_author(state: &State<ServerState>, author: Json<CreateAuthor>) -> Created<String> {
    use domain::schema::authors;

    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = state.db_pool.get().expect("Could not connect to DB");
    let author = author.into_inner();

    match diesel::insert_into(authors::table).values(&author).get_result::<Author>(&mut conn) {
        Ok(author) => {
            let response = Response { body: ResponseBody::Author(author) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}