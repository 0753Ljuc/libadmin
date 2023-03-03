use rocket::{
    form::Form,
    get, post,
    serde::json::{serde_json::json, Value}, State,
};
use rocket_auth::{Auth, Error, Login, User};
use sqlx::{query, query_as, PgPool};

use crate::{
    custom_error, database::profiles::Profiles, models::profile::ProfileForm,
    utils::helper::check_auth,
};

#[post("/user/signup", data = "<form>")]
pub async fn post_signup(
    conn: &State<PgPool>,
    auth: Auth<'_>,
    form: Form<ProfileForm>,
    profiles: &State<Profiles>,
) -> Result<(), custom_error::Error> {
    println!("signup: {form:?}");
    // should be rewrite as a transaction
    let form = form.into_inner();
    auth.signup(&form.signup).await?;
    let user_id = query!("SELECT id FROM users WHERE email = $1", &form.signup.email)
        .fetch_one(conn.inner())
        .await?
        .id;
    profiles.create_profile(&form, user_id).await?;
    auth.login(&form.signup.into()).await?;
    Ok(())
}

#[post("/user/signup", data = "<form>")]
pub async fn post_signup_v2(
    conn: &State<PgPool>,
    auth: Auth<'_>,
    form: Form<ProfileForm>,
    profiles: &State<Profiles>,
) -> Result<(), custom_error::Error> {
    let user = auth.get_user().await;
    check_auth(&user).await?;
    let user = user.unwrap();
    if !user.is_admin {
        return Err(custom_error::Error::new(
            custom_error::ErrorType::UnauthenticatedError,
            None,
        ));
    }
    println!("signup: {form:?}");
    // should be rewrite as a transaction
    let form = form.into_inner();
    auth.signup(&form.signup).await?;
    let user_id = query!("SELECT id FROM users WHERE email = $1", &form.signup.email)
        .fetch_one(conn.inner())
        .await?
        .id;
    profiles.create_profile(&form, user_id).await?;
    Ok(())
}

#[post("/user/login", data = "<form>")]
pub async fn post_login(auth: Auth<'_>, form: Form<Login>) -> Result<(), custom_error::Error> {
    auth.login(&form).await.map_err(|e| {
        println!("{e:?}");
        match e {
            Error::UnauthorizedError => {
                custom_error::Error::new(custom_error::ErrorType::WrongEmailOrPassword, None)
            }
            _ => e.into(),
        }
    })?;
    Ok(())
}

#[get("/user/logout")]
pub fn logout(auth: Auth<'_>) -> Result<(), custom_error::Error> {
    auth.logout().map_err(|e| {
        println!("{e:?}");
        e
    })?;
    Ok(())
}

#[allow(unused)]
#[get("/user/delete")]
async fn delete(auth: Auth<'_>) -> Result<(), custom_error::Error> {
    auth.delete().await?;
    Ok(())
}

#[allow(unused)]
#[get("/user/show_all_users")]
async fn show_all_users(
    conn: &State<PgPool>,
    user: Option<User>,
) -> Result<Value, custom_error::Error> {
    let users: Vec<User> = query_as("select * from users;").fetch_all(&**conn).await?;
    Ok(json!({"users": users, "user": user}))
}
