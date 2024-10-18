use domain::models::{Post, PostWithAuthor};
use domain::models::Author;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Post(Post),
    PostWithAuthor(PostWithAuthor),
    PostsWithAuthors(Vec<PostWithAuthor>),
    Posts(Vec<Post>),
    Author(Author),
    Authors(Vec<Author>),
}

#[derive(Serialize)]
pub enum ErrorResponseBody {
    Message(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    pub error: ErrorResponseBody,
}
