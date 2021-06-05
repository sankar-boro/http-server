use ahash::AHashMap;
use async_std::task::block_on;
use s4nk4r_service::{IntoServiceFactory, Service, ServiceFactory};
use crate::{App, builder::Builder, config::AppService, connection::Connection, extensions::Extensions, request::{EMPTY_HEADER, HttpRequest, Request}, resource::ResourceService, response::Response, service::{AppServiceFactory, ServiceRequest}};
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
            extensions: Rc::new(Extensions::new())
        }
    }

    fn start(&mut self) {
        let app = (self.app)();
        let a = app.into_factory();
        let b = a.new_service(());
        let c = block_on(b).unwrap();
        let d = c.services;
        d.iter().for_each(|f| {
            let g = Rc::clone(f);
            let h = g.as_ref().borrow();
            let i = h.route_name.clone();
            self.routes.insert(i, Rc::clone(&g));
        });
        self.extensions = Rc::new(a.extensions);
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
