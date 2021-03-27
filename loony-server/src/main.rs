use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub struct Builder{
  listener: TcpListener,
}

impl Builder {
  pub fn new() -> Self {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    Self {
      listener,
    }
  }

  pub fn run(&self) {
    for stream in self.listener.incoming() {
      let stream = stream.unwrap();
      handle_connection(stream);
    }
  }
}

fn handle_connection(mut stream: TcpStream){
  let mut httpdata = Vec::new();
  let mut headers = [0; 1024];
  let mut buffer = [0; 1024];
  stream.read(&mut headers).unwrap();

  loop {
    let bytes = stream.read(&mut buffer).unwrap();
    if bytes > 0 && buffer[1023] != 0 {
      httpdata.append(&mut buffer[0..1023].to_vec());
      buffer = [0; 1024];
      continue;
    }
    if bytes > 0 && buffer[1023] == 0 {
      break;
    }
  }

  println!("Headers: {:?}", headers);
  println!("Buffer: {:?}", String::from_utf8_lossy(&httpdata[..]));
  stream.write(b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\nWelcome!\r\n\r\n").unwrap();
  stream.flush().unwrap();
}

fn main() {
  let builder = Builder::new();
  builder.run();
}