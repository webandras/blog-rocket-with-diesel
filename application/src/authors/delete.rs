use shared::response_models::{Response, ResponseBody};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rocket::response::status::NotFound;
use rocket::State;
use domain::models::Author;
use infrastructure::db_pool::ServerState;

pub fn delete_author(state: &State<ServerState>, author_id: i32) -> Result<Vec<Author>, NotFound<String>> {
    use domain::schema::authors::dsl::*;
    use domain::schema::authors;

    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = state.db_pool.get().expect("Could not connect to DB");
    let response: Response;

    let num_deleted = match diesel::delete(authors.filter(id.eq(author_id))).execute(&mut conn) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error deleting author with id {} - {}", author_id, err)) };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        }
    };

    if num_deleted > 0 {
        match authors::table.select(authors::all_columns).load::<Author>(&mut conn) {
            Ok(mut authors_) => {
                authors_.sort();
                Ok(authors_)
            }
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no author with id {}", author_id)) };
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    }
}