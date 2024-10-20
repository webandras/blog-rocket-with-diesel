use shared::response_models::{Response, ResponseBody};
use application::post::{create, read, publish, delete, update};
use domain::models::{CreatePost, Post, PostWithRelations};
use rocket::{delete, get, post, put};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;

#[get("/posts")]
pub fn list_posts_handler() -> String {
    let posts: Vec<PostWithRelations> = read::list_posts();
    let response = Response { body: ResponseBody::PostsWithRelations(posts) };

    serde_json::to_string(&response).unwrap()
}

#[get("/posts/<post_id>")]
pub fn list_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let post: PostWithRelations = read::list_post(post_id)?;
    let response = Response { body: ResponseBody::PostWithRelations(post) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/posts", format = "application/json", data = "<post>")]
pub fn create_post_handler(post: Json<CreatePost>) -> Created<String> {
    create::create_post(post)
}

#[put("/posts/<post_id>", format = "application/json", data = "<post>")]
pub fn update_post_handler(post_id: i32, post: Json<Post>) -> Result<String, NotFound<String>> {
    let post: PostWithRelations = update::update_post(post_id, post)?;
    let response = Response { body: ResponseBody::PostWithRelations(post) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[put("/posts/publish/<post_id>")]
pub fn publish_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let post: PostWithRelations = publish::publish_post(post_id)?;
    let response = Response { body: ResponseBody::PostWithRelations(post) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/posts/<post_id>")]
pub fn delete_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let _posts = delete::delete_post(post_id)?;
    let response = Response { body: ResponseBody::PostsWithRelations(_posts) };

    Ok(serde_json::to_string(&response).unwrap())
}
