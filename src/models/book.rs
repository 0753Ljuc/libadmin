use chrono::{DateTime, Utc};

use super::book_category::BookCategory;

pub struct Book {
    pub bid: u32,
    pub name: String,
    pub author: String,
    pub publisher: String,
    pub pub_date: DateTime<Utc>,
    pub category: BookCategory,
    pub count: u16,
    pub price: String,
    pub remark: String,
}
