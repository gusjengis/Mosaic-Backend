use once_cell::sync::Lazy;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub static DB_POOL: Lazy<Pool<Postgres>> = Lazy::new(|| {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .connect_lazy(&database_url)
        .expect("Failed to create DB pool")
});
