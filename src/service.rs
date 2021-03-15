use futures::Future;
use crate::{route::Route};
use crate::responder::Responder;
use std::marker::PhantomData;
// use async_std::task;
// use crate::Request;
// use crate::extract::FromRequest;
use crate::web::FormDataExtractor;

pub struct ServiceConfig {
  pub routes:Vec<Route>,
}
pub trait ServiceConfigFactory {
  fn get_routes(&self) -> &Vec<Route>;
}
pub trait HttpServiceFactory {
  type Request;
  type Response;
  fn service_call(&self, param: Self::Request) -> Self::Response;
}

#[derive(Clone)]
pub(crate) struct HttpServiceFactoryWrapper<T, P, R, O> {
  factory: T,
  _t: PhantomData<(P, R, O)>
}

/**
  ......................................
  Implementations
  ......................................
*/

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

impl ServiceConfigFactory for ServiceConfig {
  fn get_routes(&self) -> &Vec<Route> {
    &self.routes
  }
}

impl<T, P, R, O> HttpServiceFactoryWrapper<T, P, R, O> 
where 
  T: Factory<P, R, O>, 
  P: FormDataExtractor,
  R: Future<Output=O>, 
  O: Responder
{
  pub fn new(factory: T) -> Self {
    Self {
      factory,
      _t: PhantomData
    }
  }
}

impl<T, P, R, O> HttpServiceFactory for HttpServiceFactoryWrapper<T, P, R, O> 
where 
  T: Factory<P, R, O>, 
  P: FormDataExtractor,
  R: Future<Output=O>, 
  O: Responder 
{
  type Request = P;
  type Response = R;

  fn service_call(&self, param: Self::Request) -> Self::Response {
    let factory = &self.factory;
    factory.call(param)
  }
}


/**
  ......................................
  This is totally something else
  ......................................
*/
// P = Parameters
// R = Returned Response
// O = Future Output type
pub trait Factory<P, R, O>: Clone + 'static
where
  // P: FromRequest,
  R: Future<Output=O>,
  O: Responder,
{
  fn call(&self, p: P) -> R;
}

impl<T, P, R, O> Factory<P, R, O> for T 
where 
  T: Fn(P) -> R + Clone + 'static, 
  // P: FromRequest, 
  R: Future<Output=O>, 
  O: Responder 
{
  fn call(&self, p: P) -> R {
    // (self)(Request{
    //   method: String::from("GET"),
    //   version: String::from("http/1.1"),
    //   url: String::from("/get"),
    // })
    (self)(p)
  }
}

/// FromRequest trait impl for tuples
macro_rules! factory_tuple ({ $(($n:tt, $T:ident)),+} => {
    impl<Func, $($T,)+ Res, O> Factory<($($T,)+), Res, O> for Func
    where Func: Fn($($T,)+) -> Res + Clone + 'static,
          Res: Future<Output=O>, 
          O: Responder 
    {
        fn call(&self, param: ($($T,)+)) -> Res {
            (self)($(param.$n,)+)
        }
    }
});

#[rustfmt::skip]
mod m {
    use super::*;

  // factory_tuple!((0, FormDataExtractor));
  // factory_tuple!((0, FormDataExtractor), (1, String));
  // factory_tuple!((0, A), (1, B), (2, C));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J));
}
