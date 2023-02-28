use rocket::{figment::Figment, Config};
use std::env;

pub const TOKEN_EXPIRE_TIME: i64 = 60 * 30;

pub const HASH_ITERATIONS: u32 = 999;

pub const BOOKS_LIMIT: i32 = 20;

pub fn generate_figment_with_env() -> Figment {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("The PORT env variable should be parsed to an integer.");
    let database_url =
        env::var("DATABASE_URL").expect("the DATABASE_URL env variable should not be empty");
    Config::figment()
        .merge(("port", port))
        .merge(("database_url", database_url))
}
