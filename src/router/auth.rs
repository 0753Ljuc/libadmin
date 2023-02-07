use rocket::post;
use rocket::serde::{json::Json, Deserialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginParams {
    pub pwd: String,
    pub username: String,
}

#[post("/login", data = "<login_params>", format = "json")]
pub fn login(login_params: Json<LoginParams>) -> String {
    println!("Hello, world!");
    format!(
        "Hello, world! Your username is {}, and password is {}",
        login_params.username, login_params.pwd
    )
}
