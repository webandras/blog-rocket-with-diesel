use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use diesel::expression_methods::ExpressionMethods;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::State;
use domain::models::{Author};
use infrastructure::db_pool::ServerState;
use shared::response_models::{Response, ResponseBody};

pub fn update_author(state: &State<ServerState>, author_id: i32, author: Json<Author>) -> Result<Author, NotFound<String>> {
    use domain::schema::authors::dsl::*;

    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = state.db_pool.get().expect("Could not connect to DB");

    match diesel::update(authors.find(author_id)).set(
        (
            firstname.eq(&author.firstname),
            lastname.eq(&author.lastname),
            email.eq(&author.email),
            is_active.eq(&author.is_active)
        )
    ).get_result::<Author>(&mut conn) {
        Ok(author) => Ok(author),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { data: ResponseBody::Error(format!("Error updating author with id {} - {}", author_id, err)) };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}