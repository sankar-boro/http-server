// use std::{collections::HashMap, io::{Read, Write}, net::TcpStream, sync::mpsc::Receiver};
// use crate::{App, service::Service};
// use crate::builder::Builder;

// type Header = [httparse::Header<'a>; 16];
// pub struct HttpService {
//   tcp_stream: TcpStream,
//   buffer: [u8; 1024],
//   headers: Header,
// }

// impl HttpService {
//   pub fn new(tcp_stream: TcpStream) -> Self {
//     Self {
//       tcp_stream,
//       buffer: [0; 1024],
//       headers: [httparse::EMPTY_HEADER; 16]
//     }
//   }

//   pub fn get_request<'a, 'b>(&'a mut self) -> httparse::Request<'a, 'b> {
//     self.tcp_stream.read(&mut self.buffer).unwrap();
//     let mut req = httparse::Request::new(&mut self.headers);
//     req.parse(&self.buffer).unwrap();
//     req
//   }
// }

pub trait AppServiceFactory {
  fn register(&mut self);
}