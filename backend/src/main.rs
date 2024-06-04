mod parser;
mod server;
mod util;
#[macro_use]
extern crate rocket;

fn main() {
    let _ = server::web_server::start_server();
   
}
