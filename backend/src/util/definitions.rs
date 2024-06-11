use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BlogPreview {
    pub title: String,
    pub preview: String,
    pub date: i64,
    pub id: String
}
#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Blog {
    pub title: String,
    pub content: String,
    pub date: i64,
}
pub static FILEPATH: &str = "/var/blogs/";
//pub static FILEPATH: &str = "../content/blogs/";
pub static FILESUFFIX: &str = ".md";
pub static FILEEXTENSION: &str = "md";