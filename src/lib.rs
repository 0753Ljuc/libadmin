use std::{collections::HashSet, env};

use rocket::{http::Method, routes};
use rocket_auth::Users;
use rocket_cors::{AllowedOrigins, Cors, CorsOptions};
use sqlx::{query_file, PgPool};

#[cfg(not(feature="mock"))]
mod config;
#[cfg(not(feature="mock"))]
mod custom_error;
#[cfg(not(feature="mock"))]
mod database;
#[cfg(not(feature="mock"))]
mod models;
#[cfg(not(feature="mock"))]
mod router;
#[cfg(not(feature="mock"))]
mod utils;

fn cors_fairing() -> Cors {
    let allowed_origins =
        AllowedOrigins::some_exact(&["http://127.0.0.1:5173", "http://localhost:5173"]);

    let allowed_methods: HashSet<rocket_cors::Method> =
        vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect();

    CorsOptions {
        allowed_origins,
        allowed_methods,
        allow_credentials: true,
        // allowed_headers: AllowedHeaders::some(&["Authorization"]),
        ..Default::default()
    }
    .to_cors()
    .expect("Cors fairing cannot be created")
}

#[cfg(not(feature="mock"))]
#[rocket::main]
pub async fn launch() -> Result<(), rocket_auth::Error> {
    dotenv::dotenv().ok();

    let figment = rocket::Config::figment()
        .merge(("port", env::var("PORT").unwrap_or(8000.to_string())))
        .merge((
            "secret_key",
            env::var("SECRET_KEY").unwrap_or_else(|_| {
                assert!(
                    cfg!(debug_assertions),
                    "The secret_key should be set in release mode."
                );
                String::new()
            }),
        ));

    let database_url =
        env::var("DATABASE_URL").expect("the DATABASE_URL env variable should not be empty");

    let conn = PgPool::connect(&database_url).await?;

    let users: Users = conn.clone().into();
    users.create_table().await?;

    create_tables(&conn).await?;

    let _ = rocket::custom(figment)
        .mount(
            "/api/v1/auth",
            routes![
                router::auth::post_signup,
                router::auth::post_login,
                router::auth::logout,
            ],
        )
        .mount(
            "/api/v1/profile",
            routes![router::profile::get_profile, router::profile::get_profiles],
        )
        .mount(
            "/api/v1/category",
            routes![
                router::category::all_categories,
                router::category::add_category,
                router::category::delete_category,
                router::category::update_category,
            ],
        )
        .mount(
            "/api/v1/book",
            routes![
                router::book::get_books,
                router::book::add_book,
                router::book::edit_book,
                router::book::delete_book,
            ],
        )
        .mount(
            "/api/v1/borrow",
            routes![
                router::borrow::new_borrow,
                router::borrow::get_borrows,
                router::borrow::return_borrow
            ],
        )
        .mount(
            "/api/v2/auth",
            routes![
                router::auth::post_signup_v2,
                router::auth::post_login,
                router::auth::logout,
            ],
        )
        .manage(conn)
        .manage(users)
        .attach(cors_fairing())
        .launch()
        .await
        .map_err(|e| {
            println!("Error: {e:?}");
        });
    Ok(())
}

async fn create_tables(conn: &PgPool) -> Result<(), sqlx::Error> {
    query_file!("migrations/2023_03_05_000001_create_profiles_table.sql")
        .execute(conn)
        .await?;
    query_file!("migrations/2023_03_05_000003_create_categories_table.sql")
        .execute(conn)
        .await?;
    query_file!("migrations/2023_03_05_000004_create_books_table.sql")
        .execute(conn)
        .await?;
    query_file!("migrations/2023_03_05_000006_create_borrow_table.sql")
        .execute(conn)
        .await?;

    Ok(())
}
