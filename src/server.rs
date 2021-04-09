use std::{collections::HashMap, io::{Read, Write}, net::TcpStream, sync::mpsc::Receiver};
use crate::{App, service::Service};
use crate::builder::Builder;

pub type AppInstance = Box<dyn Fn() -> App + 'static>;

pub struct HttpServer {
    app: AppInstance,
    builder: Builder,
    service:  HashMap<String, Box<dyn Service<Request = String, Response = String>>>,
}

impl HttpServer {
    pub fn new<F: Fn() -> App + 'static>(app: F) -> Self {
        Self { 
            app: Box::new(app), 
            builder: Builder::new(),
            service: HashMap::new(),
        }
    }

    fn start(&mut self) {
        let app = (self.app)();
        let configs = app.config.get_routes().iter();
        &app.services.iter().for_each(|app_service| {
            let service = app_service.1.new_service();
            self.service.insert(app_service.0.clone(), service);
        });

        for scope in configs {
            let mut scoped_route = String::from("");
            scoped_route.clear();
            scoped_route.push_str(&scope.scope);
            for route in scope.name.iter() {
                scoped_route.push_str(&route.0);
                let service = route.2.new_service();
                self.service.insert(scoped_route.clone(), service);
                scoped_route.clear();
                scoped_route.push_str(&scope.scope);
            }
        }
    }

    pub fn run(&mut self) {
        self.start();
        let receiver = self.builder.run();
        self.accept(receiver);
    }

    fn accept(&self, receiver: Receiver<TcpStream>) {
        loop {
            let mut stream = receiver.recv().unwrap();
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            let mut headers = [httparse::EMPTY_HEADER; 16];
            let mut req = httparse::Request::new(&mut headers);
            req.parse(&buffer).unwrap();
            let uri = req.path;
            match uri {
                Some(uri) => {
                    let service = self.service.get(uri);
                    match service {
                        Some(service) => {
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