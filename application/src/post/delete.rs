use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;
use domain::models::{Author, Post, PostWithRelations};
use domain::schema::authors;

pub fn delete_post(post_id: i32) -> Result<Vec<PostWithRelations>, NotFound<String>> {
    use domain::schema::posts::dsl::*;
    use domain::schema::posts;

    let response: Response;

    let num_deleted = match diesel::delete(posts.filter(id.eq(post_id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error deleting post with id {} - {}", post_id, err)) };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        }
    };

    if num_deleted > 0 {
        match posts::table
            .left_join(authors::table)
            .select((Post::as_select(), Option::<Author>::as_select()))
            .load::<(Post, Option<Author>)>(&mut establish_connection()) {
            Ok(result) => {
                let mut _posts: Vec<PostWithRelations> = result
                    .into_iter()
                    .map(|(post, author)| PostWithRelations { post, author })
                    .collect();

                _posts.sort();
                Ok(_posts)
            }
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no post with id {}", post_id)) };
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    }
}