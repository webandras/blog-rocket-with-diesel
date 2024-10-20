use domain::models::{Author, Post, PostWithRelations};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;
use domain::schema::authors;

pub fn list_post(post_id: i32) -> Result<PostWithRelations, NotFound<String>> {
    use domain::schema::posts;

    match posts::table
        .left_join(authors::table)
        .select((Post::as_select(), Option::<Author>::as_select()))
        .filter(posts::id.eq(post_id))
        .first::<(Post, Option<Author>)>(&mut establish_connection()) {
        Ok(tuple) => {
            let post = PostWithRelations {
                post: tuple.0,
                author: tuple.1
            };

            Ok(post)
        },
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting post with id {} - {}", post_id, err)) };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

pub fn list_posts() -> Vec<PostWithRelations> {
    use domain::schema::posts;

    match posts::table
        .left_join(authors::table)
        .select((Post::as_select(), Option::<Author>::as_select()))
        .load::<(Post, Option<Author>)>(&mut establish_connection()) {
        Ok(result) => {
            let mut posts: Vec<PostWithRelations> = result
                .into_iter()
                .map(|(post, author)| PostWithRelations { post, author })
                .collect();

            posts.sort();
            posts
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}