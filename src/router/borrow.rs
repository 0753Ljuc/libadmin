use rocket::{get, post, put, serde::json::Json, State};
use rocket_auth::Auth;
use sqlx::PgPool;

use crate::{
    custom_error::Error,
    database,
    models::borrow::{NewBorrow, ResponseBorrow},
    utils::helper::check_auth,
};

#[post("/", data = "<new_borrow>")]
pub async fn new_borrow(
    conn: &State<PgPool>,
    auth: Auth<'_>,
    new_borrow: Json<NewBorrow>,
) -> Result<(), Error> {
    let user = auth.get_user().await;
    check_auth(&user).await?;
    let user = user.unwrap();
    database::borrow::new_borrow(conn, user.id(), new_borrow.book_id)
        .await
        .map(|e| {
            println!(" in controller {e:?}");
            e
        })?;
    Ok(())
}

#[get("/")]
pub async fn get_borrows(
    conn: &State<PgPool>,
    auth: Auth<'_>,
) -> Result<Json<Vec<ResponseBorrow>>, Error> {
    let user = auth.get_user().await;
    check_auth(&user).await?;
    let user = user.unwrap();

    let borrows = match user.is_admin {
        true => database::borrow::get_all_borrows(conn).await?,
        false => database::borrow::get_borrows(conn, user.id()).await?,
    };
    Ok(Json(borrows))
}

#[put("/<book_id>")]
pub async fn return_borrow(
    conn: &State<PgPool>,
    auth: Auth<'_>,
    book_id: i32,
) -> Result<(), Error> {
    let user = auth.get_user().await;
    check_auth(&user).await?;
    database::borrow::return_borrow(conn, book_id).await?;
    Ok(())
}
