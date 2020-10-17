
use crate::route::Route;
use crate::responder::Responder;
// #[derive(Debug)]
pub struct ServiceConfig<'a, 'b> {
    pub routes:Vec<Route<'a, 'b>>,
}

impl<'a, 'b> ServiceConfig<'a, 'b> {
    pub fn service(&mut self, route: Route<'a, 'b>) {
        self.routes.push(route);
    }
}

pub trait HttpServiceFactory {}
impl<T, R> HttpServiceFactory for T where T: Fn() -> R, R:Responder {

}

impl Responder for HttpServiceFactory {}