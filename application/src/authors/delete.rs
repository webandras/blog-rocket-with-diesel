use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;
use domain::models::Author;

pub fn delete_author(author_id: i32) -> Result<Vec<Author>, NotFound<String>> {
    use domain::schema::authors::dsl::*;
    use domain::schema::authors;

    let response: Response;

    let num_deleted = match diesel::delete(authors.filter(id.eq(author_id))).execute(&mut establish_connection()) {
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
        match authors::table.select(authors::all_columns).load::<Author>(&mut establish_connection()) {
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