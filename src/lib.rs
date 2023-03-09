use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{routes, Request, Response};
use rocket_auth::Users;
use sqlx::{query_file, PgPool};
use std::env;

#[cfg(not(feature = "mock"))]
mod config;
#[cfg(not(feature = "mock"))]
mod custom_error;
#[cfg(not(feature = "mock"))]
mod database;
#[cfg(not(feature = "mock"))]
mod models;
#[cfg(not(feature = "mock"))]
mod router;
#[cfg(not(feature = "mock"))]
mod utils;

#[cfg(not(feature = "mock"))]
#[rocket::main]
pub async fn launch() -> Result<(), rocket_auth::Error> {
    dotenv::dotenv().ok();

    let figment = rocket::figment::Figment::from(rocket::Config::default())
        .merge((
            "port",
            env::var("PORT")
                .unwrap_or(8000.to_string())
                .parse::<u16>()
                .unwrap(),
        ))
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

    database::admin::create_admin(&conn, &users, "admin@163.com", "Admin123", "Admin").await?;

    create_tables(&conn).await?;

    let _ = rocket::custom(figment)
        .mount("/", routes![health])
        .mount(
            "/api/v1/auth",
            routes![router::auth::post_login, router::auth::logout,],
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
        .mount("/api/v2/auth", routes![router::auth::post_signup_v2,])
        .attach(CORS)
        .manage(conn)
        .manage(users)
        .launch()
        .await
        .map_err(|e| {
            println!("Error1 : {e:?}");
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

#[rocket::get("/health")]
async fn health() -> &'static str {
    "OK"
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PUT, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        }
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://106.55.24.94:8080",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
