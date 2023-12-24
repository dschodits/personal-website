use chrono::prelude::{DateTime, Utc};
use rocket::{
    futures::FutureExt,
    serde::{json::Json, Serialize},
};
use server::web_server;
use std::{fs::File, path::PathBuf};
use std::io::{BufRead, BufReader};
use tokio::{
    fs::{self, *},
    io::AsyncReadExt,
};

use crate::{parser::file_parser, util::definitions};
mod parser;
mod server;
mod util;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate tokio;



fn main() {
    let _ = server::web_server::start_server();
   
}
