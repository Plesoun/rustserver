fn main() {

    let get = Method::GET;
    let get = Method::DELETE;
    let get = Method::POST;
    let get = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string()); //need to convert to_string, was a literal
    server.run();
}

struct Server {
    addr: String,
}

// Self and Server are interchangeable
impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

struct Request {
    path: String,
    query_string: String,
    // use enums here instead of string
    method: Method,
}

// will be represented as ints in memory
enum Method {
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


/*
Example request

GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/
