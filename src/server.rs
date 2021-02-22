use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;


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
                Ok((mut stream, _)) => { //assign tuple, dont care about address "_"
                    println!("OK");
                    let mut buffer = [0; 1024]; //assign a buffer for an array

                    match stream.read(&mut buffer) { // buffer may be too small here
                        Ok(_) => {
                            println!("Recieved a request {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    // dbg!(buffer);
                                    dbg!(request);
                                    let response = Response::new(StatusCode::Ok, Some("<h1> Phew!! ".to_string()));
                                    write!(stream, "{}", response);
                                }
                                Err(e) => {
                                    println!("Error while parsing request {}", e);
                                }
                            }

                        },

                        Err(e) => println!("Error while reading the TCP stream {}", e)
                    }
                },
                Err(e) => println!("Error in TCP Connection {}", e)
                // "_ => " here means catch all
                // match works also on strings and others, we can use | to match multiple patterns
            }
        }
        }
    }
