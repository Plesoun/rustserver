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

            match listener.accept() {
                Ok((stream, _)) => { //assign tuple, dont care about address "_"
                    println!("OK")
                },
                Err(e) => println!("Error in TCP Connection {}", e)
                // "_ => " here means catch all
                // match works also on strings and others
            }
        }
        }
    }
