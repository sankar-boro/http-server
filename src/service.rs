
use crate::route::Route;
pub struct ServiceConfig<'a, 'b> {
    pub routes:Vec<Route<'a, 'b>>,
}

impl<'a, 'b> ServiceConfig<'a, 'b> {
    pub fn service(&mut self, route: Route<'a, 'b>) {
        self.routes.push(route);
    }
}