use std::collections::{HashSet};

use config::generate_figment_with_env;
use rocket::{catchers, http::Method, launch, routes};
use rocket_cors::{AllowedOrigins, Cors, CorsOptions};

mod auth;
mod config;
mod database;
mod models;
mod router;
mod state;

fn cors_fairing() -> Cors {
    let allowed_origins =
        AllowedOrigins::some_exact(&["http://127.0.0.1:5173", "http://localhost:5173"]);

    let allowed_methods:HashSet<rocket_cors::Method> = vec![Method::Get, Method::Post]
        .into_iter()
        .map(From::from)
        .collect();

    CorsOptions {
        allowed_origins,
        allowed_methods,
        allow_credentials: true,
        // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        ..Default::default()
    }
    .to_cors()
    .expect("Cors fairing cannot be created")
}

#[launch]
pub fn rocket() -> _ {
    dotenv::dotenv().ok();

    let figment = generate_figment_with_env();
    rocket::custom(figment)
        .mount("/api/v1/auth", routes![router::auth::login])
        .attach(cors_fairing())
        .attach(database::stage())
        .manage(state::AppState::new())
        .register(
            "/",
            catchers![
                router::error_handler::not_found,
                router::error_handler::forbidden
            ],
        )
}
