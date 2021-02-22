use server::Server;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {

    let server = Server::new("127.0.0.1:8080".to_string()); //need to convert to_string, was a literal
    server.run(WebsiteHandler);
}


/*
Example request

GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/
