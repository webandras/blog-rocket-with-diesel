use domain::models::{Author, CreateAuthor};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_author(author: Json<CreateAuthor>) -> Created<String> {
    use domain::schema::authors;

    let author = author.into_inner();

    match diesel::insert_into(authors::table).values(&author).get_result::<Author>(&mut establish_connection()) {
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