use sqlx::query;

use crate::models::profile::ProfileForm;

pub struct Profiles {
    pub conn: sqlx::Pool<sqlx::Postgres>,
}

impl From<sqlx::Pool<sqlx::Postgres>> for Profiles {
    fn from(conn: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { conn }
    }
}

impl Profiles {
    pub async fn create_profile(
        &self,
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
        .execute(&self.conn)
        .await
        .map_err(|e| {
            println!("Error: {e}");
            e
        })?;
        Ok(())
    }
}
