use rocket::{figment::Figment, Config};
use std::env;

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
