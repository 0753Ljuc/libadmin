

use rocket::serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Admin {
    pub id: i32,
    pub name: String,
    pub gender: String, // n,f,m
    pub phone_number: String,
    pub id_card: Option<i32>,
    pub borrow_card: Option<i32>,
    pub permission: bool,
    pub hash: String,
}



