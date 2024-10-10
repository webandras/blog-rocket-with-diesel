use domain::models::Author;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn list_author(author_id: i32) -> Result<Author, NotFound<String>> {
    use domain::schema::authors;

    match authors::table.find(author_id).first::<Author>(&mut establish_connection()) {
        Ok(author) => Ok(author),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting author with id {} - {}", author_id, err)) };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

pub fn list_authors() -> Vec<Author> {
    use domain::schema::authors;

    match authors::table.select(authors::all_columns).load::<Author>(&mut establish_connection()) {
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