use ahash::AHashMap;
use crate::{App, builder::Builder, config::AppService, connection::Connection, extensions::Extensions, request::{EMPTY_HEADER, Request}, resource::ResourceService, response::Response, service::AppServiceFactory};
use std::{cell::{RefCell}, net::TcpStream, rc::Rc, sync::mpsc::Receiver};

static RES_OK: &str = "HTTP/1.1 200 OK\r\n\r\n";
static RES_NF: &str = "HTTP/1.1 401 NOT FOUND\r\n\r\nNOT FOUND";

pub type AppInstance = Box<dyn Fn() -> App + 'static>;
pub struct HttpServer {
    app: AppInstance,
    builder: Builder,
    routes: AHashMap<String, Rc<RefCell<ResourceService>>>,
    extensions: Rc<Extensions>,
}

impl HttpServer {
    pub fn new<F: Fn() -> App + 'static>(app: F) -> Self {
        Self { 
            app: Box::new(app), 
            builder: Builder::new(),
            routes: AHashMap::new(),
            extensions: Rc::new(Extensions::new()),
        }
    }

    fn start(&mut self) {
        let mut app = (self.app)();
        let mut app_service = AppService::new();
        app.register(&mut app_service);
        if let Some(factories) = app.factories {
            let factories = factories;
            for factory in factories {
                self.routes.insert(factory.path.clone(), Rc::new(RefCell::new(factory)));
            }
        }
        self.extensions = Rc::new(app.extensions);
    }

    pub fn run(&mut self) {
        self.start();
        let a = self.builder.run();
        self.accept(a);
    }

    fn accept(&self, receiver: Receiver<TcpStream>) {
        let res = Response::new(&self.routes, self.extensions.clone());
        loop {
            let mut buffer = [0; 1024];
            let stream = receiver.recv().unwrap();
            let mut conn = Connection::new(stream);
            conn.read(&mut buffer);
            let mut headers = [EMPTY_HEADER; 16];
            let mut req = Request::new(&mut headers);
            req.parse(&buffer);

            
            let r = res.build(&req);
            match r {
                Ok(r) => {
                    let mut res = String::from("");
                    res.push_str(RES_OK);
                    res.push_str(&r);
                    conn.write(&res);
                }
                Err(_) => {
                    conn.write(RES_NF);
                }
            }

            conn.close();
        }
    }
}
