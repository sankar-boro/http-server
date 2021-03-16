mod wrapper;

use std::marker::PhantomData;
use wrapper::{Wrapper, Service, Extract, BoxedRouteService, BoxedRouteNewService, RouteNewService};
pub trait Responder {
  fn respond(self) -> String;
}
pub trait FromRequest{
  fn callme(self) -> Self;
}
// *************************************************

struct Route{
  route: BoxedRouteNewService
}
// **************************************************

impl Responder for String {
  fn respond(self) -> String {
    self
  }
}

impl FromRequest for String {
  fn callme(self) -> Self {
      self
  }
}
pub trait Factory<Arg, Res> {
  fn factory_call(&self, param: Arg) -> Res;
}

impl<T, String, Res> Factory<String, Res> for T 
where 
  T: Fn(String) -> Res + Clone + 'static, 
  Res: Responder
{
  fn factory_call(&self, param: String) -> Res {
    (self)(param)
  }
}

fn index(param: String) -> impl Responder {
  "Hello".to_string()
}

fn run<T, Arg, Res>(factory: T) -> Route
where 
  T: Factory<Arg, Res> + 'static,
  Arg: FromRequest + 'static,
  Res: Responder + 'static,
{
  // let w = Extract::new(Wrapper::new(factory));
  let route = RouteNewService::new(Extract::new(Wrapper::new(factory)));
  Route {
    route: Box::new(route),
  };
  todo!();
}

fn main() {
  let route = run(index);
  let r = route.route;
}