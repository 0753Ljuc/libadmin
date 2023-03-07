use rocket::serde::{Deserialize, Serialize};
use rocket::FromForm;
use rocket_auth::Signup;

#[derive(Debug, FromForm)]
pub struct ProfileForm {
    pub username: String,
    pub gender: i32,
    pub signup: Signup,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Profile {
    pub username: String,
    pub gender: i32,
    pub user_id: Option<i32>,
    pub profile_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub profile_type: i32,
    pub email: String,
}
