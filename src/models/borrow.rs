#[derive(serde::Serialize)]
pub struct ResponseBorrow {
    pub borrow_id: i32,
    pub book_id: i32,
    pub user_id: i32,
    pub borrow_date: chrono::NaiveDate,
    pub book_name: String,
    pub borrow_status: i32,
    pub username: String,
}

pub struct Borrow {
    pub borrow_id: i32,
    pub book_id: i32,
    pub user_id: i32,
    pub borrow_date: chrono::NaiveDate,
    pub borrow_status: i32,
}

#[derive(serde::Deserialize)]
pub struct NewBorrow {
    pub book_id: i32,
}
