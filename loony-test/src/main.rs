mod service;
mod handler;
mod responder;
mod extract;
mod request;
mod route;

use std::future::Future;
use extract::FromRequest;
use request::HttpRequest;
use responder::Responder;
use route::BoxedRouteServiceFactory;
use handler::{Factory, Handler, Extract, RouteServiceFactory};
use service::ServiceRequest;

struct Routes {
    routes: Vec<BoxedRouteServiceFactory>,
}

impl Routes {
    fn new() -> Self {
        Routes {
            routes: Vec::new()
        }
    }

    fn route<T, P, R, O>(&mut self, factory: T) 
    where
        T: Factory<P, R, O> + 'static,
        P: FromRequest + 'static,
        R: Future<Output=O> + 'static,
        O: Responder + 'static,
    {
        let a: Handler<T, P, R, O> = Handler::new(factory);
        let b = Extract::new(a);
        let c = RouteServiceFactory::new(b);
        self.routes.push(Box::new(c));
    }
}

async fn index(param: String) -> String {
    let mut data = String::from("");
    data.push_str("Hello ");
    data.push_str(&param);
    data
}

#[tokio::main]
async fn main() {
    let mut routes = Routes::new();
    routes.route(index);
    for r in routes.routes.iter() {
        let a: &BoxedRouteServiceFactory = &r;
        let b = a.new_service(());
        let mut c = b.await.unwrap();
        let sr = ServiceRequest(HttpRequest{url: "World!".to_string()});
        let d = c.call(sr).await.unwrap();
        println!("{}", d.0.value);
    }
}