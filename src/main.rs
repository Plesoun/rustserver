fn main() {

    let string = String::from("127.0.0.1:8080");
    // borrow the slice of an existing string allocated in memory
    let string_slice = &string[10..]; // give me everything after tenth byte (some ASCII characters like emojis occupy more than one byte)
    let string_borrow: &str = &string;
    // this is known at the compile time, strings can be expanded or shrink at runtime, string slices are immutable
    let string_literal = "123";
    // another slicing option



    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);

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
