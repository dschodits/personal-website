use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BlogPreview {
    pub title: String,
    pub preview: String,
    pub date: i64,
}
#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Blog {
    pub title: String,
    pub content: String,
    pub date: i64,
}
pub static FILEPATH: &str = "../content/test/";
pub static FILESUFFIX: &str = ".md";