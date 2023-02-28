use rocket::{delete, get, post, put, serde::json::Json, State};
use rocket_auth::Auth;
use sqlx::PgPool;

use crate::{
    config::BOOKS_LIMIT,
    custom_error::{Error, ErrorType},
    database,
    models::book::{Book, EditBook, NewBook},
    utils::helper::check_auth,
};

#[derive(serde::Serialize)]
pub struct BooksResponse {
    pub books: Vec<Book>,
    pub total: i64,
    pub from: i32,
    pub limit: i32,
}
#[get("/?<from>&<limit>&<book_name>&<book_author>&<book_category>")]
pub async fn get_books(
    conn: &State<PgPool>,
    auth: Auth<'_>,
    from: Option<i32>,
    limit: Option<i32>,
    book_name: Option<&str>,
    book_author: Option<&str>,
    book_category: Option<&str>,
) -> Result<Json<BooksResponse>, Error> {
    check_auth(&auth.get_user().await).await?;
    let (books, total) = database::book::get_books(
        conn,
        from.unwrap_or(0),
        limit.unwrap_or(BOOKS_LIMIT),
        book_name,
        book_author,
        book_category,
    )
    .await?;

    Ok(Json::from(BooksResponse {
        books,
        total,
        from: from.unwrap_or(0),
        limit: limit.unwrap_or(BOOKS_LIMIT),
    }))
}

#[post("/", data = "<book>", format = "json")]
pub async fn add_book(
    conn: &State<PgPool>,
    auth: Auth<'_>,
    book: Json<NewBook>,
) -> Result<(), Error> {
    let user = auth.get_user().await;
    check_auth(&user).await?;
    let user = user.unwrap();
    if !user.is_admin {
        return Err(Error::new(ErrorType::UnauthenticatedError, None));
    }
    database::book::add_book(conn, book.into_inner()).await?;
    Ok(())
}

#[put("/", data = "<book>", format = "json")]
pub async fn edit_book(
    conn: &State<PgPool>,
    auth: Auth<'_>,
    book: Json<EditBook>,
) -> Result<(), Error> {
    let user = auth.get_user().await;
    check_auth(&user).await?;
    let user = user.unwrap();
    if !user.is_admin {
        return Err(Error::new(ErrorType::UnauthenticatedError, None));
    }
    database::book::edit_book(conn, book.into_inner()).await?;
    Ok(())
}

#[delete("/<book_id>")]
pub async fn delete_book(conn: &State<PgPool>, auth: Auth<'_>, book_id: i32) -> Result<(), Error> {
    let user = auth.get_user().await;
    check_auth(&user).await?;
    let user = user.unwrap();
    if !user.is_admin {
        return Err(Error::new(ErrorType::UnauthenticatedError, None));
    }
    database::book::delete_book(conn, book_id).await?;
    Ok(())
}
