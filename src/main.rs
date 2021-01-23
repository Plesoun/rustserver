use server::Server;
use http::request::Request;

fn main() {

    let server = Server::new("127.0.0.1:8080".to_string()); //need to convert to_string, was a literal
    server.run();
}

mod server {

    pub struct Server {
        addr: String,
    }

    // Self and Server are interchangeable
    impl Server {
        pub fn new(addr: String) -> Self {
            Self {
                addr
            }
        }

        pub fn run(self) {
            println!("Listening on {}", self.addr);
        }
    }
}

mod http {
    pub mod request {

        use super::method::Method;
        
        pub struct Request {
            path: String,
            query_string: Option<String>, //can be None or some, it is a way to express absence of a value in a type-safe way (no no pointer exceptions)
            // use enums here instead of string
            method: Method,
        }
    }
    pub mod method {
        // will be represented as ints in memory
        pub enum Method {
            GET,
            DELETE,
            POST,
            PUT,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }
    }
}


/*
Example request

GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/
