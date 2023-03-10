use rocket_db_pools::Database;

pub mod book;
pub mod borrow;
pub mod category;
pub mod profile;
pub mod profiles;

#[derive(Database)]
#[database("sqlx")]
pub struct Db(sqlx::PgPool);
