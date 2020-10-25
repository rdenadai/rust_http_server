use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handler_request(&mut self, request: &Request) -> Response;

    fn handler_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse a request: {}", e);
        Response::new(StatusCode::NotFound, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            // Usefull, but with a bigger error handle is bad
            //let res = listener.accept();
            // if res.is_err() {
            //     continue;
            // }
            // let (stream, addr) = res.unwrap();
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 2048];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Receive a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(&request);
                                    handler.handler_request(&request)
                                }
                                Err(e) => handler.handler_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e)
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        }
    }
}
