use std::rc::Rc;
use std::cell::RefCell;
use std::net::TcpStream;
use std::io::{Read, Write};
use bytes::{Bytes, BytesMut, Buf, BufMut};
use http::header;

pub struct Connection {
    stream: Rc<RefCell<TcpStream>>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: Rc<RefCell<TcpStream>>) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(2024),
        }
    }

    pub fn build(&mut self) {
        let mut buffer = [0; 1024];
        let mut stream = self.stream.borrow_mut();
        stream.read(&mut buffer[..]).unwrap();

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);
        req.parse(&mut buffer).unwrap();
        println!("{:?}", headers.clone());
        self.buffer.put(&buffer[..]);
        let res = format!("HTTP/1.1 200 OK\r\n\r\nHello World!");


        stream.write(res.as_bytes()).unwrap();            
        stream.flush().unwrap();
        stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}