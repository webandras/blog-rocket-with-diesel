use diesel::{QueryDsl, RunQueryDsl};
use diesel::expression_methods::ExpressionMethods;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use domain::models::{Author};
use infrastructure::establish_connection;
use shared::response_models::{Response, ResponseBody};

pub fn update_author(author_id: i32, author: Json<Author>) -> Result<Author, NotFound<String>> {
    use domain::schema::authors::dsl::*;

    match diesel::update(authors.find(author_id)).set(
        (
            firstname.eq(&author.firstname),
            lastname.eq(&author.lastname),
            email.eq(&author.email),
            is_active.eq(&author.is_active)
        )
    ).get_result::<Author>(&mut establish_connection()) {
        Ok(author) => Ok(author),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error updating author with id {} - {}", author_id, err)) };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}