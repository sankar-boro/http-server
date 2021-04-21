use std::{borrow::Borrow, net::TcpStream, sync::mpsc::Receiver};
use crate::{App, app::AppServiceFactory};
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
        let mut app = (self.app)();
        app.register();
        if let Some(factories) = app.factories {
            let factories = factories;
            for factory in factories {
                let mut factory = factory;
                let res = &mut factory.call("".to_string());
                println!("Route: {}\nResponse: {}", factory.path, res);
            }
        }
    }

    pub fn run(&mut self) {
        self.start();
    }

    fn accept(&self, _: Receiver<TcpStream>) {
    }
}