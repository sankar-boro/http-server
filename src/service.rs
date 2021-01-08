use std::env;
use futures::Future;
use crate::route::Route;
use crate::responder::Responder;
use crate::web::FormDataExtractor;

struct Arguments {
    name: Option<String>,
    age: Option<String>,
}

pub struct ServiceConfig {
  pub routes:Vec<Route>,
}

impl ServiceConfig {
  pub fn new() -> Self {
    ServiceConfig {
      routes: Vec::new(),
    }
  }
	
	pub fn service(&mut self, route: Route) {
    self.routes.push(route);
  }
}

pub trait ServiceConfigFactory {
  fn get_routes(&self) -> &Vec<Route>;
}

impl ServiceConfigFactory for ServiceConfig {
  fn get_routes(&self) -> &Vec<Route> {
    &self.routes
  }
}

pub trait HttpServiceFactory {
  fn get_response(&self) -> String;
}

// impl<T, R> HttpServiceFactory for T 
// where 
//   T: Fn() -> R, R:Responder 
// {
//   fn get_response(&self) -> String {
//     let data = self();
//     data.respond()
//   }
// }

// impl<T, R, L> HttpServiceFactory for T 
// where 
//   T: Fn(L) -> R, R:Responder 
// {
//   fn get_response(&self) -> String {
//     let data = self();
//     data.respond()
//   }
// }

pub(crate) struct ServiceFactoryWrapper<T> {
  route: String,
  factory: Option<T>,
}

impl<T> ServiceFactoryWrapper<T> {
  pub fn new(route:&str, factory: T) -> Self {
    Self {
      route: route.to_owned(),
      factory: Some(factory),
    }
  }
}

pub trait AppServiceFactory {
  fn get_route(&self) -> String;
  fn get_response(&self) -> String;
}

impl<T> AppServiceFactory for ServiceFactoryWrapper<T>
where
  T: HttpServiceFactory,
{
  fn get_route(&self) -> String {
    self.route.clone()
  }

  fn get_response(&self) -> String {
    if let Some(f) = &self.factory {
      return f.get_response();
    }
    return String::from("");
  }
}

pub trait Factory<T, R>
where
    R: Responder
{
    fn call(&self, param: T) -> R;
}

impl<F, R> Factory<(), R> for F
where
    F: Fn() -> R + Clone + 'static,
    R: Responder
{
    fn call(&self, _: ()) -> R {
        (self)()
    }
}

/// FromRequest trait impl for tuples
macro_rules! factory_tuple ({ $(($n:tt, $T:ident)),+} => {
    impl<Func, $($T,)+ Res> Factory<($($T,)+), Res> for Func
    where Func: Fn($($T,)+) -> Res,
          Res: Responder,
    {
        fn call(&self, param: ($($T,)+)) -> Res {
            (self)($(param.$n,)+)
        }
    }
});

#[rustfmt::skip]
mod m {
    use super::*;

  factory_tuple!((0, FormDataExtractor));
  // factory_tuple!((0, A), (1, B));
  // factory_tuple!((0, A), (1, B), (2, C));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J));
}
