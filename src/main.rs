use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod server;
mod http;
mod website_handler;

fn main() {
	// run with ROOT=$(pwd)/root cargo run or use default
	let default_path = format!("{}/root", env!("CARGO_MANIFEST_DIR"));
	let public_path = env::var("ROOT").unwrap_or(default_path);
	println!("Current path is {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string()); //need to convert to_string, was a literal
    server.run(WebsiteHandler::new(public_path));
}


/*
Example request

GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/
