use rocket_auth::Users;
use sqlx::{query, PgPool};

pub async fn create_admin(
    conn: &PgPool,
    users: &Users,
    email: &str,
    pwd: &str,
    username: &str,
) -> Result<(), rocket_auth::Error> {
    if users.get_by_email(email).await.is_err() {
        users.create_user(email, pwd, true).await?;
        let id = users.get_by_email(email).await?.id();
        query!(
            "INSERT INTO profiles(username, gender, user_id, profile_type) VALUES($1, $2, $3, $4)",
            username,
            1,
            id,
            1
        )
        .execute(conn)
        .await
        .map_err(|e| {
            println!("Error: {e}");
            e
        })?;
    }

    Ok(())
}
