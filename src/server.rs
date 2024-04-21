use ahash::AHashMap;
use async_std::task::block_on;
use loony_service::{IntoServiceFactory, ServiceFactory};
use crate::{App, app_service::AppHttpService, builder::Builder, connection::Connection, extensions::Extensions, request::{EMPTY_HEADER, Request}, resource::ResourceService, response::Response};
use std::{cell::RefCell, marker::PhantomData, net::TcpStream, rc::Rc, sync::mpsc::Receiver};

static RES_OK: &str = "HTTP/1.1 200 OK\r\n\r\n";
static RES_NF: &str = "HTTP/1.1 401 NOT FOUND\r\n\r\nNOT FOUND";

pub type AppInstance = Box<dyn Fn() -> App + 'static>;
pub struct HttpServer<F, I, T> 
where F: Fn() -> I + Send + Clone + 'static,
I: IntoServiceFactory<T>,
T: ServiceFactory,
{
    app: F,
    builder: Builder,
    routes: AHashMap<String, Rc<RefCell<ResourceService>>>,
    extensions: Rc<Extensions>,
    _p: PhantomData<T>
}


impl<F, I, T> HttpServer<F, I, T> 
where F: Fn() -> I + Send + Clone + 'static,
    I: IntoServiceFactory<T>,
    T: ServiceFactory<Request=(), Config = (), Service = AppHttpService>,
{
    pub fn new(app: F) -> Self {
        Self { 
            app, 
            builder: Builder::new(),
            routes: AHashMap::new(),
            extensions: Rc::new(Extensions::new()),
            _p: PhantomData,
        }
    }

    fn start(&mut self) {
        let app = (self.app)();
        let app_factory = app.into_factory();
        let app_service = app_factory.new_service(());
        let http_service:Result<AppHttpService, <T as ServiceFactory>::InitError> = block_on(app_service);
        if let Ok(http_service) = http_service {
            let exts = http_service.extensions;
            self.routes = http_service.routes;
            self.extensions = Rc::new(exts);
        };
    }

    pub fn run(&mut self) {
        self.start();
        let a = self.builder.run();
        println!("Http Server is running on Port: 3005");
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
