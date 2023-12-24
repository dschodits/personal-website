use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Blog {
    pub title: String,
    pub content: String,
    pub date: String,
}