use std::net::TcpListener;

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

        // returns result enum
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            let result = listener.accept();

            if result.is_err() {
                continue;
            }

            let (stream, addr) = result.unwrap();
        }
        }
    }
