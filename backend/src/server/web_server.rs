use chrono::prelude::{DateTime, Utc};
use rocket::{
    futures::FutureExt,
    serde::{json::Json, Serialize},
};
use std::io::{BufRead, BufReader};
use std::{fs::File, path::PathBuf};
use tokio::{
    fs::{self, *},
    io::AsyncReadExt,
};

use crate::parser::file_parser;
use crate::util::definitions;



#[get("/blogs")]
#[allow(dead_code)]
async fn get_all_blogs() -> Result<Json<Vec<definitions::Blog>>, String> {
    let mut output = Vec::new();

    let mut test = read_dir("../content/test/").await.unwrap();
    while let Some(item) = test.next_entry().await.unwrap() {
        let entry = file_parser::get_file_object_from_path(item.path());
        match entry {
            Some(entry_value) => output.push(entry_value),
            None => continue,
        }
    }
    print!("{:?}", output);

    Ok(Json(output))
}


#[rocket::main]
pub async fn start_server() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/blogs", routes![get_all_blogs])
        .ignite().await?
        .launch().await?;

    Ok(())
}