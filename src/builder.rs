use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, SocketAddr};
// use socket2::{Socket, Domain, Type};
use std::fs::File;
use std::io::prelude::*;

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


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let mut file = match File::open("foo.txt") {
        Ok(file) => file,
        Err(_) => File::create("file.txt").unwrap()
    };

    file.write_all(&buffer[..]).unwrap();


    // let mut headers = [httparse::EMPTY_HEADER; 16];
    // let mut req = httparse::Request::new(&mut headers);
    // req.parse(&buffer).unwrap();
    // for header in req.headers.iter() {    
    //     println!("{}: {}", header.name,  String::from_utf8_lossy(header.value));
    // }
    // println!("\n\n\n");
}
