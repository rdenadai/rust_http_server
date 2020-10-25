use super::StatusCode;
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    // pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
    // Dynamic dispatch implemented with dyn keyword... runtime cost
    // pub fn send(&self, stream: &mut dyn Write) -> IoResult<()> {
    // Static dispatch implemented with impl keyword... bigger binary, compilation cost
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason(),
            body
        )
    }
}
