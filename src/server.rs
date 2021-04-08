use std::{collections::HashMap, io::{Read, Write}};
use crate::App;
use crate::builder::Builder;

pub type AppInstance = Box<dyn Fn() -> App + 'static>;

pub struct HttpServer {
    app: AppInstance,
    builder: Builder,
}

impl HttpServer {
    pub fn new<F: Fn() -> App + 'static>(app: F) -> Self {
        Self { 
            app: Box::new(app), 
            builder: Builder::new() 
        }
    }

    pub fn run(&self) {
        let mut ext = HashMap::new();
        let app = (self.app)();
        let configs = app.config.get_routes().iter();
        &app.services.iter().for_each(|data| {
            ext.insert(data.0.clone(), &data.1);
        });

        for scope in configs {
            let mut r = String::from("");
            r.push_str(&scope.scope);
            for route in scope.name.iter() {
                r.push_str(&route.0);
                let s = &route.2;
                ext.insert(r.clone(), s);
                r.clear();
                r.push_str(&scope.scope);
            }
        }
        let r = self.builder.run();
        loop {
            let mut s = r.recv().unwrap();
            let mut buffer = [0; 2048];
            s.read(&mut buffer).unwrap();
            let mut headers = [httparse::EMPTY_HEADER; 16];
            let mut req = httparse::Request::new(&mut headers);
            req.parse(&buffer).unwrap();
            let uri = req.path;
            match uri {
                Some(uri) => {
                    let m = ext.get(uri);
                    match m {
                        Some(m) => {
                            let sd = m.new_service();
                            let nsd = sd.call("".to_string());
                            let res = format!("HTTP/1.1 200 OK\r\n\r\n{}", &nsd);
                            s.write(res.as_bytes()).unwrap();
                        }
                        None => {
                            let res = format!("HTTP/1.1 200 OK\r\n\r\nOops!");
                            s.write(res.as_bytes()).unwrap();
                        }
                    }
                }
                None => {
                    let res = format!("HTTP/1.1 200 OK\r\n\r\nOops!");
                    s.write(res.as_bytes()).unwrap();
                }
            }
            s.flush().unwrap();
            s.shutdown(std::net::Shutdown::Both).unwrap();
        }
    }
}