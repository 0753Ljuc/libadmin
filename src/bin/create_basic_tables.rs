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
        let users: Users = conn.clone().into();
        users
            .create_table()
            .await
            .map_err(|_| sqlx::Error::RowNotFound)?;

        query_file!("migrations/2023_03_05_000001_create_profiles_table.sql")
            .execute(&conn)
            .await?;
        query_file!("migrations/2023_03_05_000003_create_categories_table.sql")
            .execute(&conn)
            .await?;
        query_file!("migrations/2023_03_05_000004_create_books_table.sql")
            .execute(&conn)
            .await?;
        query_file!("migrations/2023_03_05_000006_create_borrow_table.sql")
            .execute(&conn)
            .await?;
    }
    Ok(())
}
