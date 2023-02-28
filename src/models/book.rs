use rocket::FromForm;
use sqlx::FromRow;

#[derive(serde::Serialize, serde::Deserialize, FromRow)]
pub struct Book {
    pub book_id: i32,
    pub book_name: String,
    pub book_author: String,
    pub book_publisher: String,
    pub book_price: sqlx::types::Decimal,
    pub book_description: String,
    pub category_id1: Option<i32>,
    pub category_id2: Option<i32>,
    pub category_id3: Option<i32>,
    pub status: i32, // 0: available, 1: borrowed, 2: reserved
}

#[derive(serde::Deserialize, Debug)]
pub struct NewBook {
    pub book_name: String,
    pub book_author: String,
    pub book_publisher: String,
    pub book_price: sqlx::types::Decimal,
    pub book_description: String,
    pub book_category: Vec<i32>,
}

#[derive(serde::Deserialize, Debug)]
pub struct EditBook {
    pub book_id: i32,
    pub book_name: Option<String>,
    pub book_author: Option<String>,
    pub book_publisher: Option<String>,
    pub book_price: Option<sqlx::types::Decimal>,
    pub book_description: Option<String>,
    pub book_category: Option<Vec<i32>>,
}

impl EditBook {
    pub fn get_update_sql(&self) -> String {
        let mut update_message = String::new();
        if let Some(book_name) = &self.book_name {
            update_message = format!("{update_message}book_name='{book_name}' ");
        }
        if let Some(book_author) = &self.book_author {
            update_message = format!("{update_message}book_author='{book_author}' ");
        }
        if let Some(book_publisher) = &self.book_publisher {
            update_message = format!("{update_message}book_publisher='{book_publisher}' ");
        }
        if let Some(book_price) = self.book_price {
            update_message = format!("{update_message}book_price='{book_price}' ");
        }
        if let Some(book_description) = &self.book_description {
            update_message = format!("{update_message}book_description='{book_description}' ");
        }
        if let Some(book_category) = &self.book_category {
            book_category
                .iter()
                .enumerate()
                .for_each(|(index, category)| {
                    update_message = format!(
                        "{update_message}category_id{id}='{category}' ",
                        id = index + 1
                    );
                })
        }
        let mut sql = "UPDATE books".to_string();
        if !update_message.is_empty() {
            sql = format!("{sql} SET {update_message}");
        };
        println!("sql: {sql}");
        format!("{sql} WHERE book_id={}", &self.book_id)
    }
}
