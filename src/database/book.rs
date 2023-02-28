use sqlx::{Pool, Postgres};

use crate::{
    custom_error::{Error, ErrorType},
    models::book::{Book, EditBook, NewBook},
};

pub async fn get_books(
    conn: &Pool<Postgres>,
    from: i32,
    limit: i32,
    book_name: Option<&str>,
    book_author: Option<&str>,
    book_category: Option<&str>,
) -> Result<(Vec<Book>, i64), Error> {
    let mut sql = "SELECT * FROM books".to_string();
    let mut count_sql = "SELECT COUNT(*) FROM books".to_string();
    let mut condition: Vec<String> = vec![];
    if let Some(book_name) = book_name {
        condition.push(format!("book_name LIKE '%{book_name}%'"));
    };
    if let Some(book_author) = book_author {
        condition.push(format!("book_author LIKE '%{book_author}%'"));
    };
    if let Some(book_category) = book_category {
        condition.push(format!("(category_id1={book_category} OR category_id2={book_category} OR category_id3={book_category})"));
    };
    if !condition.is_empty() {
        let constraints = condition.join(" AND ");
        sql = format!("{sql} WHERE {constraints}");
        count_sql = format!("{count_sql} WHERE {constraints}");
    };

    sql = format!("{sql} ORDER BY book_id ASC LIMIT {limit} OFFSET {from}");
    println!("sql: {sql}");
    println!("count_sql: {count_sql}");

    let books = sqlx::query_as::<Postgres, Book>(&sql)
        .fetch_all(conn)
        .await
        .map_err(|e| {
            println!("Error: {e}");
            e
        })?;
    let count = sqlx::query_scalar::<Postgres, i64>(&count_sql)
        .fetch_one(conn)
        .await
        .map_err(|e| {
            println!("Error: {e}");
            e
        })?;
    Ok((books, count))
}

pub async fn add_book(conn: &Pool<Postgres>, newbook: NewBook) -> Result<(), Error> {
    sqlx::query!(
        r#" INSERT INTO books (
    book_name, 
    book_author, 
    book_publisher, 
    book_price, 
    book_description, 
    category_id1, 
    category_id2, 
    category_id3
  ) 
  VALUES ( $1, $2, $3, $4, $5, $6, $7, $8)"#,
        newbook.book_name,
        newbook.book_author,
        newbook.book_publisher,
        newbook.book_price,
        newbook.book_description,
        newbook.book_category.get(0),
        newbook.book_category.get(1),
        newbook.book_category.get(2),
    )
    .execute(conn)
    .await
    .map_err(|e| {
        println!("err: {e}");
        e
    })?;
    Ok(())
}

pub async fn edit_book(conn: &Pool<Postgres>, book: EditBook) -> Result<(), Error> {
    let sql = book.get_update_sql();
    sqlx::query(&sql).execute(conn).await?;
    Ok(())
}

pub async fn delete_book(conn: &Pool<Postgres>, book_id: i32) -> Result<(), Error> {
    sqlx::query!("DELETE FROM books WHERE book_id=$1", book_id)
        .execute(conn)
        .await?;
    Ok(())
}
