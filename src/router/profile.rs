// restful api

use rocket::{get, serde::json::Json, State};
use rocket_auth::Auth;
use sqlx::PgPool;

use crate::{
    custom_error::{Error, ErrorType},
    database::profile,
    models::profile::Profile,
    utils::helper::check_auth,
};

#[get("/")]
pub async fn get_profile(conn: &State<PgPool>, auth: Auth<'_>) -> Result<Json<Profile>, Error> {
    let user = auth.get_user().await;
    check_auth(&user).await?;
    let user = user.unwrap();
    let profile = profile::get_profile(conn, user.id()).await?;
    Ok(Json::from(profile))
}

#[get("/all")]
pub async fn get_profiles(
    conn: &State<PgPool>,
    auth: Auth<'_>,
) -> Result<Json<Vec<Profile>>, Error> {
    let user = auth.get_user().await;
    check_auth(&user).await?;
    let user = user.unwrap();
    if !user.is_admin {
        return Err(Error::new(ErrorType::UnauthenticatedError, None));
    };
    let profiles = profile::get_profiles(conn).await?;
    Ok(Json::from(profiles))
}
