use std::{
    net::TcpStream, 
    sync::mpsc::Receiver
};
use crate::App;
use crate::builder::Builder;
use loony_service::Service;

pub type AppInstance = Box<dyn Fn() -> App + 'static>;

pub struct HttpServer {
    app: AppInstance,
    builder: Builder,
}

impl HttpServer {
    pub fn new<F: Fn() -> App + 'static>(app: F) -> Self {
        Self { 
            app: Box::new(app), 
            builder: Builder::new(),
        }
    }

    fn start(&mut self) {
        let app = (self.app)();
        let services = app.services;
        for service in services.iter() {
            let mut new_service = service.new_service();
            let res = new_service.call("Request: ".to_string());
            println!("{}", res);
        }
    }

    pub fn run(&mut self) {
        self.start();
    }

    fn accept(&self, _: Receiver<TcpStream>) {
    }
}