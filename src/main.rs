fn main() {

    let get = Method::GET;
    let delete = Method::DELETE("look, I contain data!".to_string());
    let post = Method::POST;
    let put = Method::PUT;


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
    GET, //can specify the actual int representation in memory, we can also use them to contain data
    DELETE(String), //we can also use them to contain data
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
