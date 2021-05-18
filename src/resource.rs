use crate::DB;
use crate::route::Route;
use loony_service::{ServiceFactory, Service};
use crate::route::RouteService;

pub struct Resource {
  prefix: String,
  route: Route,
}

impl Resource {
  pub fn new(prefix: String) -> Self {
    Resource {
      prefix,
      route: Route::new(""),
    }
  }

  pub fn route(mut self, route: Route) -> Self {
    self.route = route;
    self
  }
}

impl ServiceFactory for Resource {
    type Request = DB;
    type Response = String;
    type Error = ();
    type Service = ResourceService;

    fn new_service(&self) -> Self::Service {
        let mut path = self.prefix.clone();
        path.push_str(&self.route.path);
        let route = self.route.new_service();
        ResourceService {
          len: path.len() as u16,
          path,
          route,
        }
    }
}

pub struct ResourceService {
    pub route: RouteService,
    pub path: String,
    pub len: u16,
}

impl Service for ResourceService {
    type Request = DB;
    type Response = String;
    type Error = ();

    fn call(&mut self, req: Self::Request) -> Self::Response {
        self.route.call(req)
    }
}
