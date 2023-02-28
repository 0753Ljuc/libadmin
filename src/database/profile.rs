use sqlx::{query_as, Pool, Postgres};

use crate::{custom_error::Error, models::profile::Profile};

pub async fn get_profile(conn: &Pool<Postgres>, user_id: i32) -> Result<Profile, Error> {
    Ok(query_as!(
        Profile,
        "SELECT profiles.*, users.email FROM profiles INNER JOIN users 
        ON profiles.user_id=users.id WHERE profiles.user_id = $1",
        user_id
    )
    .fetch_one(conn)
    .await
    .map_err(|e| {
        println!("Error: {e}");
        e
    })?)
}

pub async fn get_profiles(conn: &Pool<Postgres>) -> Result<Vec<Profile>, Error> {
    Ok(query_as!(
        Profile,
        "SELECT profiles.*, users.email FROM profiles INNER JOIN users 
        ON profiles.user_id=users.id WHERE $1",
        true
    )
    .fetch_all(conn)
    .await?)
}
