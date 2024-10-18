use shared::response_models::{Response, ResponseBody};
use application::post::{create, read, publish, delete, update};
use domain::models::{Post, NewPost, PostWithAuthor};
use rocket::{delete, get, post, put};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;

#[get("/posts")]
pub fn list_posts_handler() -> String {
    let posts: Vec<PostWithAuthor> = read::list_posts();
    let response = Response { body: ResponseBody::PostsWithAuthors(posts) };

    serde_json::to_string(&response).unwrap()
}

#[get("/post/<post_id>")]
pub fn list_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let post = read::list_post(post_id)?;
    let response = Response { body: ResponseBody::Post(post) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/post", format = "application/json", data = "<post>")]
pub fn create_post_handler(post: Json<NewPost>) -> Created<String> {
    create::create_post(post)
}

#[put("/post/<post_id>", format = "application/json", data = "<post>")]
pub fn update_post_handler(post_id: i32, post: Json<NewPost>) -> Result<String, NotFound<String>> {
    let post = update::update_post(post_id, post)?;
    let response = Response { body: ResponseBody::Post(post) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[put("/post/publish/<post_id>")]
pub fn publish_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let post = publish::publish_post(post_id)?;
    let response = Response { body: ResponseBody::Post(post) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/post/<post_id>")]
pub fn delete_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let posts = delete::delete_post(post_id)?;
    let response = Response { body: ResponseBody::Posts(posts) };

    Ok(serde_json::to_string(&response).unwrap())
}
