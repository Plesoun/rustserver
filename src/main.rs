fn main() {

    let server = Server::new("127.0.0.1:8080");
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

    }
}
