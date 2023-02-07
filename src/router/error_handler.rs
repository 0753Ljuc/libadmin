use rocket::{
    catch,
    serde::json::{serde_json::json, Value},
};

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(403)]
pub fn forbidden() -> Value {
    json!({
        "status": "error",
        "reason": "You are not allowed to access this resource."
    })
}
