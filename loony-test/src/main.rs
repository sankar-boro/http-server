mod wrapper;

use std::fmt::{Error, Write};
use wrapper::{Wrapper, Service, Extract, BoxedRouteNewService, RouteNewService};

fn writer<W: Write>(f: &mut W, s: &str) -> Result<(), Error> {
    f.write_fmt(format_args!("{}", s))
}

pub trait Responder {
  fn respond(self) -> String;
}
pub trait FromRequest{
  fn from_request(req: String) -> Self;
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
  fn from_request(req: String) -> Self {
    req
  }
}
pub trait Factory<Arg, Res>: Clone + 'static {
  fn factory_call(&self, param: Arg) -> Res;
}

impl<T, String, Res> Factory<String, Res> for T 
where 
  T: Fn(String) -> Res + Clone + 'static, 
  Res: Responder
{
  fn factory_call(&self, param: String) -> Res {
    println!("Factory called");
    (self)(param)
  }
}

fn index(param: String) -> impl Responder {
  let mut buf = String::new();
  writer(&mut buf, "Hello World! ").unwrap();
  writer(&mut buf, &param).unwrap();
  buf
}

fn run<T, Arg, Res>(factory: T) -> Route
where 
  T: Factory<Arg, Res> + Clone + 'static,
  Arg: FromRequest + 'static,
  Res: Responder + 'static,
{
  let route = RouteNewService::new(Extract::new(Wrapper::new(factory)));
  Route {
    route: Box::new(route),
  }
}

fn main() {
  let route = run(index);
  let r = route.route;
  let n = r.new_service();
  let final_res = n.call("This is Sankar".to_owned());
  print!("{}", final_res);
}