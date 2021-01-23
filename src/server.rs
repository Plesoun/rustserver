pub struct Server {
    addr: String,
}

// Self and Server are interchangeable
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
    }
}
