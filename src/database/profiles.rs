use sqlx::{query, PgPool};

use crate::models::profile::ProfileForm;

pub async fn create_profile(
    conn: &PgPool,
    profile: &ProfileForm,
    user_id: i32,
) -> Result<(), sqlx::Error> {
    query!(
        "INSERT INTO profiles(username, gender, user_id, profile_type) VALUES($1, $2, $3, $4)",
        profile.username,
        profile.gender,
        user_id,
        0
    )
    .execute(conn)
    .await
    .map_err(|e| {
        println!("Error: {e}");
        e
    })?;
    Ok(())
}
