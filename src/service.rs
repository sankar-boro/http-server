
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
pub struct ServiceFactoryWrapper<T> {
    factory: Option<T>,
}

impl<T> ServiceFactoryWrapper<T> {
    pub fn new(factory: T) -> Self {
        Self {
            factory: Some(factory),
        }
    }
}
pub trait AppServiceFactory {
}
impl<T> AppServiceFactory for ServiceFactoryWrapper<T>
where
    T: HttpServiceFactory,
{
}

impl Responder for HttpServiceFactory {}


