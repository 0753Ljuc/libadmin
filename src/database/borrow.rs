use sqlx::{Pool, Postgres, Transaction};

use crate::custom_error::{Error, ErrorType};
use crate::models::book::Book;
use crate::models::borrow::{Borrow, ResponseBorrow};

pub async fn new_borrow(conn: &Pool<Postgres>, user_id: i32, book_id: i32) -> Result<(), Error> {
    let mut transaction = conn.begin().await?;

    let book = get_book_by_id(&mut transaction, book_id).await?;

    match book.status {
        1 => return Err(Error::new(ErrorType::BookIsRent, None)),
        2 => return Err(Error::new(ErrorType::BookNotAvailable, None)),
        _ => (),
    }

    sqlx::query!("UPDATE books SET status=1 WHERE book_id=$1", book_id)
        .execute(&mut transaction)
        .await?;

    sqlx::query!(
        "INSERT INTO borrows (user_id, book_id) VALUES ($1, $2)",
        user_id,
        book_id
    )
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn get_borrows(
    conn: &Pool<Postgres>,
    user_id: i32,
) -> Result<Vec<ResponseBorrow>, Error> {
    let borrows = sqlx::query_as!(
        ResponseBorrow,
        "SELECT borrows.borrow_id, borrows.book_id, borrows.user_id, profiles.username,
         borrows.borrow_status, borrows.borrow_date, books.book_name
        FROM borrows 
        INNER JOIN books ON borrows.book_id=books.book_id
        INNER JOIN users ON borrows.user_id=users.id
        INNER JOIN profiles ON users.id=profiles.user_id
        WHERE borrows.user_id=$1",
        user_id
    )
    .fetch_all(conn)
    .await?;

    Ok(borrows)
}

pub async fn get_all_borrows(conn: &Pool<Postgres>) -> Result<Vec<ResponseBorrow>, Error> {
    let borrows = sqlx::query_as!(
        ResponseBorrow,
        "SELECT borrows.borrow_id, borrows.book_id, borrows.user_id, profiles.username,
        borrows.borrow_status, borrows.borrow_date, books.book_name
        FROM borrows 
        INNER JOIN books ON borrows.book_id=books.book_id 
        INNER JOIN users ON borrows.user_id=users.id
        INNER JOIN profiles ON users.id=profiles.user_id
        WHERE $1",
        true
    )
    .fetch_all(conn)
    .await?;

    Ok(borrows)
}

pub async fn return_borrow(conn: &Pool<Postgres>, book_id: i32) -> Result<(), Error> {
    let mut transaction = conn.begin().await?;
    let book = get_book_by_id(&mut transaction, book_id).await?;
    match book.status {
        2 => return Err(Error::new(ErrorType::BookNotAvailable, Some("书本维护中"))),
        0 => {
            return Err(Error::new(
                ErrorType::BookNotAvailable,
                Some("书本已经归还"),
            ))
        }
        _ => (),
    }

    let borrow_item = sqlx::query_as!(
        Borrow,
        "SELECT * FROM borrows WHERE book_id=$1 ORDER BY borrow_date DESC LIMIT 1",
        book_id
    )
    .fetch_one(&mut transaction)
    .await?;

    sqlx::query!("UPDATE books SET status=0 WHERE book_id=$1", book_id)
        .execute(&mut transaction)
        .await?;

    sqlx::query!(
        "UPDATE borrows SET borrow_status=1 WHERE borrow_id=$1",
        borrow_item.borrow_id
    )
    .execute(&mut transaction)
    .await?;

    transaction.commit().await?;
    Ok(())
}

async fn get_book_by_id(
    transaction: &mut Transaction<'_, Postgres>,
    book_id: i32,
) -> Result<Book, Error> {
    let book = sqlx::query_as!(Book, "SELECT * FROM books WHERE book_id=$1", book_id)
        .fetch_one(transaction)
        .await
        .map_err(|_| Error::new(ErrorType::BookNotFOund, None))?;

    Ok(book)
}
