use sqlx::Pool;
use sqlx::Postgres;

use crate::custom_error::Error as CustomError;
use crate::models::category::Category;

pub async fn all_categories(conn: &Pool<Postgres>) -> Result<Vec<Category>, CustomError> {
    Ok(sqlx::query_as!(
        Category,
        "SELECT * FROM categories ORDER BY category_id ASC"
    )
    .fetch_all(conn)
    .await
    .map_err(|e| {
        println!("Error: {e}");
        e
    })?)
}

pub async fn add_category(conn: &Pool<Postgres>, category_name: &str) -> Result<(), CustomError> {
    sqlx::query!(
        "INSERT INTO categories (category_name) VALUES ($1)",
        category_name
    )
    .execute(conn)
    .await
    .map_err(|e| {
        println!("Error: {e}");
        e
    })?;
    Ok(())
}

pub async fn delete_category(conn: &Pool<Postgres>, category_id: i32) -> Result<(), CustomError> {
    sqlx::query!("DELETE FROM categories WHERE category_id = $1", category_id)
        .execute(conn)
        .await
        .map_err(|e| {
            println!("Error: {e}");
            e
        })?;
    Ok(())
}

pub async fn update_category(
    conn: &Pool<Postgres>,
    category_id: i32,
    category_name: &str,
) -> Result<(), CustomError> {
    sqlx::query!(
        "UPDATE categories SET category_name = $1 WHERE category_id = $2",
        category_name,
        category_id
    )
    .execute(conn)
    .await
    .map_err(|e| {
        println!("Error: {e}");
        e
    })?;
    Ok(())
}

pub async fn get_category_by_name(
    conn: &Pool<Postgres>,
    category_name: &str,
) -> Result<Category, CustomError> {
    let category = sqlx::query_as!(
        Category,
        "SELECT * FROM categories WHERE category_name = $1",
        category_name
    )
    .fetch_one(conn)
    .await
    .map_err(|e| {
        println!("Error: {e}");
        e
    })?;

    Ok(category)
}
