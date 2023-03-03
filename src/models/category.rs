






#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cotegories(pub Vec<Category>);

#[derive(serde::Deserialize)]
pub struct NewCategory {
    pub category_name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Category {
    pub category_id: i32,
    pub category_name: String,
}
