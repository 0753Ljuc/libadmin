use crate::{
    custom_error::{Error as CustomError, ErrorType},
    database::{self, Db},
    models::category::{Category, NewCategory},
    utils::helper::check_auth,
};
use rocket::{delete, get, post, put, serde::json::Json, State};
use rocket_auth::Auth;
use sqlx::PgPool;

#[get("/")]
pub async fn all_categories(
    conn: &State<PgPool>,
    auth: Auth<'_>,
) -> Result<Json<Vec<Category>>, CustomError> {
    check_auth(&auth.get_user().await).await?;
    let categories = database::category::all_categories(conn).await?;
    Ok(Json::from(categories))
}

#[post("/", data = "<new_category>", format = "json")]
pub async fn add_category(
    conn: &State<PgPool>,
    new_category: Json<NewCategory>,
    auth: Auth<'_>,
) -> Result<(), CustomError> {
    check_auth(&auth.get_user().await).await?;
    let item = database::category::get_category_by_name(conn, &new_category.category_name).await;
    if item.is_ok() {
        return Err(CustomError::new(ErrorType::CategoryAlreadyExists, None));
    }
    database::category::add_category(conn, &new_category.category_name).await?;
    Ok(())
}

#[delete("/<category_id>")]
pub async fn delete_category(
    conn: &State<PgPool>,
    auth: Auth<'_>,
    category_id: i32,
) -> Result<(), CustomError> {
    check_auth(&auth.get_user().await).await?;
    database::category::delete_category(conn, category_id).await?;
    Ok(())
}

#[put("/<category_id>", data = "<new_category>", format = "json")]
pub async fn update_category(
    conn: &State<PgPool>,
    auth: Auth<'_>,
    category_id: i32,
    new_category: Json<NewCategory>,
) -> Result<(), CustomError> {
    check_auth(&auth.get_user().await).await?;
    let item = database::category::get_category_by_name(conn, &new_category.category_name).await;
    if item.is_ok() {
        return Err(CustomError::new(ErrorType::CategoryAlreadyExists, None));
    }
    database::category::update_category(conn, category_id, &new_category.category_name).await?;
    Ok(())
}
