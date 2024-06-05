
use rocket::{
    fairing::{Fairing, Info, Kind}, http::{Header, Status}, serde::json::Json, Request, Response
};


use tokio::{
    fs::{*},
};

use crate::parser::{file_parser, html_parser};
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
    let mut entry = file_parser::get_blog_from_path(blog_name);
    
        match entry {
            Ok(mut entry_value) => {
                entry_value.content = html_parser::content_to_html(&entry_value.content).unwrap();
                return Ok(Json(entry_value))
            },
            Err(_fehler) => return Err(Status::NotFound)
        }

}


#[rocket::main]
pub async fn start_server() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(CORS)
        .mount("/", routes![get_all_blogs])
        .mount("/", routes![get_specific_blog])
        .ignite().await?
        .launch().await?;

    Ok(())
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    }
}