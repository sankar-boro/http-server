use crate::route::Route;
use loony_service::{ServiceFactory, Service};
use crate::route::RouteService;

pub trait HttpServiceFactory {
  fn register(self);
}

pub struct Resource {
  path: String,
  route: Route,
}

impl Resource {
  pub fn new(path: &str, scope: &str) -> Self {
    let mut route = String::from("");
    route.push_str(scope);
    route.push_str(path);
    Resource {
      path: route,
      route: Route::new(),
    }
  }

  pub fn route(mut self, route: Route) -> Self {
    self.route = route;
    self
  }
}

impl HttpServiceFactory for Resource
{
    fn register(self) {
      
    }
}


impl ServiceFactory for Resource {
    type Request = String;
    type Response = String;
    type Error = ();
    type Service = ResourceService;

    fn new_service(&self) -> Self::Service {
        let route = self.route.new_service();
        ResourceService {
          path: self.path.clone(),
          route
        }
    }
}

pub struct ResourceService {
    pub route: RouteService,
    pub path: String,
}

impl Service for ResourceService {
    type Request = String;
    type Response = String;
    type Error = ();

    fn call(&mut self, req: Self::Request) -> Self::Response {
        self.route.call(req)
    }
}
