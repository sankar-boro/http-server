use std::net::TcpStream;
use std::io::{Read, Write};

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) {
        self.stream.read(buffer).unwrap();
    }

    pub fn write(&mut self, buffer: &str) {
        self.stream.write(buffer.as_bytes()).unwrap();
    }

    pub fn close(&mut self) {
        self.stream.flush().unwrap();
        self.stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}