use shared::response_models::{Response, ResponseBody};
use application::authors::{create, read, delete, update};
use domain::models::{Author, CreateAuthor};
use rocket::{delete, get, post, put, State};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;
use rocket_validation::Validated;
use infrastructure::db_pool::ServerState;

#[get("/authors")]
pub fn list_authors_handler(state: &State<ServerState>) -> String {
    let authors: Vec<Author> = read::list_authors(&state);
    let response = Response { data: ResponseBody::Authors(authors) };

    serde_json::to_string(&response).unwrap()
}

#[get("/authors/<author_id>")]
pub fn list_author_handler(state: &State<ServerState>, author_id: i32) -> Result<String, NotFound<String>> {
    let author = read::list_author(&state, author_id)?;
    let response = Response { data: ResponseBody::Author(author) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/authors/<author_id>/posts")]
pub fn list_author_posts_handler(state: &State<ServerState>, author_id: i32) -> Result<String, NotFound<String>> {
    let posts = read::list_author_posts(&state, author_id)?;
    let response = Response { data: ResponseBody::Posts(posts) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/authors", format = "application/json", data = "<author>")]
pub fn create_author_handler(state: &State<ServerState>, author: Validated<Json<CreateAuthor>>) -> Created<String> {
    let author = author.into_inner();
    create::create_author(&state, author)
}

#[put("/authors/<author_id>", format = "application/json", data = "<author>")]
pub fn update_author_handler(state: &State<ServerState>, author_id: i32, author: Json<Author>) -> Result<String, NotFound<String>> {
    let author = update::update_author(&state, author_id, author)?;
    let response = Response { data: ResponseBody::Author(author) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/authors/<author_id>")]
pub fn delete_author_handler(state: &State<ServerState>, author_id: i32) -> Result<String, NotFound<String>> {
    let authors = delete::delete_author(&state, author_id)?;
    let response = Response { data: ResponseBody::Authors(authors) };

    Ok(serde_json::to_string(&response).unwrap())
}

