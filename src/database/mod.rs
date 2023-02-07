use rocket::{
    error,
    fairing::{self, AdHoc},
    routes, Build, Rocket,
};
use rocket_db_pools::{
    sqlx::{self},
    Database,
};
use sqlx::postgres::PgPoolOptions;

use crate::router;
pub mod schema;

#[derive(Database)]
#[database("sqlx")]
pub struct Db(sqlx::PgPool);

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Sqlx Stage", |rocket| async {
        rocket
            .attach(AdHoc::try_on_ignite("SQLX init", init_db))
            // .attach(AdHoc::try_on_ignite("SQLX Migrations", run_migrations))
            .mount("/", routes![router::admin::get_admin])
    })
}

pub async fn init_db(rocket: Rocket<Build>) -> fairing::Result {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    Ok(rocket.manage(Db(pool)))
}
