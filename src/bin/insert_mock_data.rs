#[rocket::main]
async fn main() -> Result<(), sqlx::Error> {
    #[cfg(feature = "mock")]
    {
        use rocket_auth::Users;
        use sqlx::{query_file, PgPool};
        use std::env;

        dotenv::dotenv().ok();

        let database_url =
            env::var("DATABASE_URL").expect("the DATABASE_URL env variable should not be empty");

        let conn = PgPool::connect(&database_url).await?;

        query_file!("migrations/2023_03_05_000014_delete_books.sql")
            .execute(&conn)
            .await?;

        query_file!("migrations/2023_03_05_000013_delete_categories.sql")
            .execute(&conn)
            .await?;

        query_file!("migrations/2023_03_05_000011_insert_categories.sql")
            .execute(&conn)
            .await?;

        query_file!("migrations/2023_03_05_000012_insert_books.sql")
            .execute(&conn)
            .await?;
    }
    Ok(())
}
