use shared::response_models::{Response, ResponseBody};
use application::authors::{create, read, delete, update};
use domain::models::{Author, CreateAuthor};
use rocket::{delete, get, post, put};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;
use rocket_validation::Validated;

#[get("/authors")]
pub fn list_authors_handler() -> String {
    let authors: Vec<Author> = read::list_authors();
    let response = Response { body: ResponseBody::Authors(authors) };

    serde_json::to_string(&response).unwrap()
}

#[get("/authors/<author_id>")]
pub fn list_author_handler(author_id: i32) -> Result<String, NotFound<String>> {
    let author = read::list_author(author_id)?;
    let response = Response { body: ResponseBody::Author(author) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/authors/<author_id>/posts")]
pub fn list_author_posts_handler(author_id: i32) -> Result<String, NotFound<String>> {
    let posts = read::list_author_posts(author_id)?;
    let response = Response { body: ResponseBody::Posts(posts) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/authors", format = "application/json", data = "<author>")]
pub fn create_author_handler(author: Validated<Json<CreateAuthor>>) -> Created<String> {
    let author = author.into_inner();
    create::create_author(author)
}

#[put("/authors/<author_id>", format = "application/json", data = "<author>")]
pub fn update_author_handler(author_id: i32, author: Json<Author>) -> Result<String, NotFound<String>> {
    let author = update::update_author(author_id, author)?;
    let response = Response { body: ResponseBody::Author(author) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/authors/<author_id>")]
pub fn delete_author_handler(author_id: i32) -> Result<String, NotFound<String>> {
    let authors = delete::delete_author(author_id)?;
    let response = Response { body: ResponseBody::Authors(authors) };

    Ok(serde_json::to_string(&response).unwrap())
}

