use std::{cell::RefCell, net::TcpStream, rc::Rc, sync::mpsc::Receiver};
use crate::{App, app::AppServiceFactory, resource::ResourceService};
use crate::builder::Builder;
use std::io::{Read, Write};
use loony_service::Service;
use ahash::AHashMap;

pub type AppInstance = Box<dyn Fn() -> App + 'static>;

pub struct HttpServer {
    app: AppInstance,
    builder: Builder,
    routes: AHashMap<String, Rc<RefCell<ResourceService>>>,
}

impl HttpServer {
    pub fn new<F: Fn() -> App + 'static>(app: F) -> Self {
        Self { 
            app: Box::new(app), 
            builder: Builder::new(),
            routes: AHashMap::new(),
        }
    }

    fn start(&mut self) {
        let mut app = (self.app)();
        app.register();
        if let Some(factories) = app.factories {
            let factories = factories;
            for factory in factories {
                self.routes.insert(factory.path.clone(), Rc::new(RefCell::new(factory)));
            }
        }
    }

    pub fn run(&mut self) {
        self.start();
        let a = self.builder.run();
        self.accept(a);
    }

    fn accept(&self, receiver: Receiver<TcpStream>) {
        loop {
            let mut stream = receiver.recv().unwrap();
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            let mut headers = [httparse::EMPTY_HEADER; 16];
            let mut req = httparse::Request::new(&mut headers);
            req.parse(&buffer).unwrap();
            match req.path {
                Some(uri) => {
                    let service = self.routes.get(uri);
                    match service {
                        Some(service) => {
                            let mut service = service.borrow_mut();
                            let res = service.call("".to_string());
                            let res = format!("HTTP/1.1 200 OK\r\n\r\n{}", &res);
                            stream.write(res.as_bytes()).unwrap();
                        }
                        None => {
                            let res = format!("HTTP/1.1 200 OK\r\n\r\nOops!");
                            stream.write(res.as_bytes()).unwrap();
                        }
                    }
                }
                None => {
                    let res = format!("HTTP/1.1 200 OK\r\n\r\nOops!");
                    stream.write(res.as_bytes()).unwrap();
                }
            }
            stream.flush().unwrap();
            stream.shutdown(std::net::Shutdown::Both).unwrap();
        }
    }
}