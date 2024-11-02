use domain::models::{Post, PostWithRelations};
use domain::models::Author;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    #[serde(rename(serialize = "message"))]
    Message(String),

    #[serde(rename(serialize = "error"))]
    Error(String),

    #[serde(rename(serialize = "post"))]
    Post(Post),

    #[serde(rename(serialize = "post"))]
    PostWithRelations(PostWithRelations),

    #[serde(rename(serialize = "posts"))]
    PostsWithRelations(Vec<PostWithRelations>),

    #[serde(rename(serialize = "posts"))]
    Posts(Vec<Post>),

    #[serde(rename(serialize = "author"))]
    Author(Author),

    #[serde(rename(serialize = "authors"))]
    Authors(Vec<Author>),
}

#[derive(Serialize)]
pub enum ErrorResponseBody {
    #[serde(rename(serialize = "message"))]
    Message(String),

    #[serde(rename(serialize = "error"))]
    Error(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub data: ResponseBody,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    pub error: ErrorResponseBody,
}
