use crate::route::Route;
use std::sync::Arc;
use loony_service::{ServiceFactory, IntoServiceFactory, Service};
use crate::route::RouteService;

pub trait HttpServiceFactory {
  fn register(self);
}

pub struct Resource {
  path: String,
  route: Route,
  factory_ref: Option<ResourceFactory>,
}

impl Resource {
  pub fn new(path: &str) -> Self {
    Resource {
      path: path.to_owned(),
      route: Route::new(),
      factory_ref: None,
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

// impl<T> IntoServiceFactory<T> for Resource
// where
//     T: ServiceFactory<
//         Request = String,
//         Response = String,
//         Error = (),
//     >,
// {
//     fn into_factory(self) -> T {
//         self.factory_ref = Some(ResourceFactory {
//             route: self.route,
//         });
//         self
//     }
// }

pub struct ResourceFactory {
    route: Route,
}

impl ServiceFactory for ResourceFactory {
    type Request = String;
    type Response = String;
    type Error = ();
    type Service = ResourceService;

    fn new_service(&self) -> Self::Service {
        let route = self.route.new_service();
        ResourceService {
          route
        }
    }
}

pub struct ResourceService {
    route: RouteService,
}

impl Service for ResourceService {
    type Request = String;
    type Response = String;
    type Error = ();

    fn call(&mut self, mut req: Self::Request) -> Self::Response {
        self.route.call(req)
    }
}