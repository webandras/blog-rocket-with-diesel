use domain::models::{Post, PostWithRelations};
use domain::models::Author;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
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
