use std::env;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;


pub type DbPool = Pool<ConnectionManager<PgConnection>>;


pub struct ServerState {
    pub db_pool: DbPool
}

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .max_size(10)
        .build(manager)
        .expect("Could not build connection pool")
}

