use std::{pin::Pin, task::{Context, Poll}};

use crate::{DB, route::{BoxedRouteService, RouteFutureService}, service::{ServiceRequest, ServiceResponse}};
use crate::route::Route;
use loony_service::{ServiceFactory, Service};
use crate::route::RouteService;
// use std::future::{Ready};
use futures::{Future, FutureExt, future::{Ready, ok}};

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
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = ();
    type Service = ResourceService;
    type Future = CreateResourceService;
    type InitError = ();
    type Config = ();
 
    fn new_service(&self, _: ()) -> Self::Future {
        let mut path = self.prefix.clone();
        path.push_str(&self.route.path);
        let fut = self.route.new_service(());
        CreateResourceService {
          len: path.len() as u16,
          path,
          fut,
        }
    }
}
#[pin_project::pin_project]
pub struct CreateResourceService {
    #[pin]
    pub fut: RouteFutureService,
    pub path: String,
    pub len: u16,
}
pub struct ResourceService {
    pub service: BoxedRouteService,
    pub path: String,
    pub len: u16,
}

impl Service for ResourceService {
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = ();
    type Future = Pin<Box<dyn Future<Output=Result<ServiceResponse, ()>>>>;

    fn call(&mut self, req: Self::Request) -> Self::Future {
        self.service.call(req).boxed_local()
    }
}

impl Future for CreateResourceService {
    type Output = Result<ResourceService, ()>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.fut.fut.as_mut().poll(cx) {
          Poll::Ready(service) => {
            let a =Poll::Ready(Ok(ResourceService {
                service: service.unwrap(),
                path: self.path.clone(),
                len: self.len,
            }));
            return a;
          },
          Poll::Pending => Poll::Pending
        }
    }
}
