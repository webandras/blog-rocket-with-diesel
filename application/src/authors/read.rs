use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use domain::models::{Author, Post};
use shared::response_models::{Response, ResponseBody};
use rocket::response::status::NotFound;
use rocket::State;
use infrastructure::db_pool::ServerState;

pub fn list_author(state: &State<ServerState>, author_id: i32) -> Result<Author, NotFound<String>> {
    use domain::schema::authors;

    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = state.db_pool.get().expect("Could not connect to DB");

    match authors::table.find(author_id).first::<Author>(&mut conn) {
        Ok(author) => Ok(author),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { data: ResponseBody::Error(format!("Error selecting author with id {} - {}", author_id, err)) };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

pub fn list_author_posts(state: &State<ServerState>, author_id: i32) -> Result<Vec<Post>, NotFound<String>> {
    use domain::schema::authors;

    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = state.db_pool.get().expect("Could not connect to DB");

    let author = match authors::table
        .filter(authors::id.eq(author_id))
        .select(Author::as_select())
        .first(&mut conn) {
        Ok(author) => Ok(author),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { data: ResponseBody::Error(format!("Error selecting author with id {} - {}", author_id, err)) };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }?;

    match Post::belonging_to(&author).select(Post::as_select()).load(&mut conn) {
        Ok(author_post) => Ok(author_post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { data: ResponseBody::Error(format!("Error selecting author with id {} - {}", author_id, err)) };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

pub fn list_authors(state: &State<ServerState>) -> Vec<Author> {
    use domain::schema::authors;

    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = state.db_pool.get().expect("Could not connect to DB");

    match authors::table.select(authors::all_columns).load::<Author>(&mut conn) {
        Ok(mut authors) => {
            authors.sort();
            authors
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
