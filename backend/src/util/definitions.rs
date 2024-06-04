use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BlogPreview {
    pub title: String,
    pub content: String,
    pub date: String,
}
pub static FILEPATH: &str = "../content/test/";