
use rocket::{
    serde::{json::Json},
    http::{Status}
};


use tokio::{
    fs::{*},
};

use crate::parser::file_parser;
use crate::util::definitions;



#[get("/blogs")]
#[allow(dead_code)]
async fn get_all_blogs() -> Result<Json<Vec<definitions::BlogPreview>>, Status> {
    let mut output = Vec::new();

    let mut test = read_dir(definitions::FILEPATH).await.unwrap();
    while let Some(item) = test.next_entry().await.unwrap() {
        let entry = file_parser::get_file_object_from_path(item.file_name().to_str().unwrap());
        match entry {
            Ok(entry_value) => output.push(entry_value),
            Err(_fehler) => return Err(Status::NotFound)
        }
    }
    print!("{:?}", output);

    Ok(Json(output))
}

#[get("/blog/<blog_name>")]
#[allow(dead_code)]
async fn get_specific_blog(blog_name: &str) -> Result<Json<definitions::Blog>, Status> {
    print!("HELP");
    let entry = file_parser::get_blog_from_path(blog_name);
        match entry {
            Ok(entry_value) => return Ok(Json(entry_value)),
            Err(_fehler) => return Err(Status::NotFound)
        }

}


#[rocket::main]
pub async fn start_server() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![get_all_blogs])
        .mount("/", routes![get_specific_blog])
        .ignite().await?
        .launch().await?;

    Ok(())
}