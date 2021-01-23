fn main() {

    let string = String::from("127.0.0.1:8080");
    // borrow the slice of an existing string allocated in memory
    let string_slice = &string[10..];
    dbg!(&string);
    dbg!(string_slice);


    //let server = Server::new("127.0.0.1:8080");
    //server.run();
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
