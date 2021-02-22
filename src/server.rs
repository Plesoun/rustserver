use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::convert::TryInto;


pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Did not manage to parse the request, got this thing: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

// Self and Server are interchangeable
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
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

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e)
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Did not manage to send a response: {}", e);
                            }

                        }

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
