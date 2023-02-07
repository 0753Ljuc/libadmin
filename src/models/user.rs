pub enum Gender {
    Male,
    Female,
}
pub struct User {
    pub uid: u32,
    pub name: String,
    pub gender: Gender,
    pub phone_number: u16,
    pub id_card: u32,
    pub borrow_card: u32,
    pub permission: bool,
}
