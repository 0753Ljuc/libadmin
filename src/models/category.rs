use ::chrono::Duration;
use rocket::serde::{Deserialize, Serialize};
use serde::ser::SerializeSeq;
use sqlx::types::chrono;

use crate::config::TOKEN_EXPIRE_TIME;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cotegories(pub Vec<Category>);

#[derive(serde::Deserialize)]
pub struct NewCategory {
    pub category_name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Category {
    pub category_id: i32,
    pub category_name: String,
}
