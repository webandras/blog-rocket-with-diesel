use std::env;
use core::time::Duration;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

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
        .connection_timeout(Duration::new(30, 0))
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn run_migrations(db_pool: &DbPool) {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = db_pool.get().expect("Could not connect to DB");

    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

