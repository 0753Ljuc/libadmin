use crate::{database::Db, models, state::AppState};
use rocket::{
    get,
    serde::json::{serde_json::json, Value},
    State,
};
use rocket_db_pools::Connection;

#[get("/admin/<id>")]
pub async fn get_admin(mut db: Connection<Db>, state: &State<AppState>, id: i32) -> Value {
    let _secret = state.secret.clone();

    match sqlx::query_as!(
        models::admin::Admin,
        "SELECT * FROM admins WHERE id = $1",
        id
    )
    .fetch_one(&mut *db)
    .await
    {
        Ok(admin) => {
            println!("\n\nAdmin: {admin:#?}");
            json!(admin)
        }
        Err(e) => {
            println!("\n\nError: {e:#?}");
            panic!("Error: {e:#?}")
        }
    }
}
