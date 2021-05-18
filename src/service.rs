use async_std::task;
use std::future::Future;
use std::marker::PhantomData;

use crate::DB;
use crate::{FromRequest};
use crate::responder::Responder;
use crate::route::{BoxedRouteService};
use loony_service::{Service, ServiceFactory};

pub trait Factory<Arg, Res, O>: Clone + 'static 
where 
  Res: Future<Output=O>, 
  O: Responder
{
  fn factory_call(&self, param: Arg) -> Res;
}

impl<T, A, Res, O> Factory<(A,), Res, O> for T 
where 
  T: Fn(A,) -> Res + Clone + 'static, 
  Res: Future<Output=O>,
  O: Responder,
{
  fn factory_call(&self, (one,): (A,)) -> Res {
    (self)(one)
  }
}

impl<T, Res, O> Factory<(), Res, O> for T 
where 
  T: Fn() -> Res + Clone + 'static, 
  Res: Future<Output=O>,
  O: Responder,
{
  fn factory_call(&self, _: ()) -> Res {
    (self)()
  }
}

impl<T, A, B, Res, O> Factory<(A,B,), Res, O> for T 
where 
  T: Fn(A, B,) -> Res + Clone + 'static, 
  Res: Future<Output=O>,
  O: Responder,
{
  fn factory_call(&self, (one, two,): (A, B,)) -> Res {
    (self)(one, two)
  }
}

// Structs
pub struct Wrapper<T, Arg, Res, O> 
where 
  T: Factory<Arg, Res, O>,
  Res: Future<Output=O>,
  O: Responder,
{
  service: T,
  _t: PhantomData<(Arg, Res, O)>
}

pub struct Extract<Arg: FromRequest, S> {
  service: S,
  _t: PhantomData<Arg>
}

pub struct ExtractService<Arg: FromRequest, S> {
    service: S,
    _t: PhantomData<Arg>,
}

struct RouteServiceWrapper<T: Service> {
    service: T,
}

pub struct RouteServiceFactory<T>
where
  T: ServiceFactory<
    Request=DB
  >,
  T::Service: 'static,
{
  service: T,
}

/**
* Implementations
*
*/


// Struct Implementation

impl<T, Arg, Res, O> Clone for Wrapper<T, Arg, Res, O>
where
  T: Factory<Arg, Res, O>,
  Res: Future<Output=O>,
  O: Responder,
{
    fn clone(&self) -> Self {
      Self {
        service: self.service.clone(),
        _t: PhantomData,
      }
    }
}

impl<T, Arg, Res, O> Wrapper<T, Arg, Res, O> 
where 
  T: Factory<Arg, Res, O>,
  Res: Future<Output=O>,
  O: Responder,
  {
    // service: Fn(Arg) -> Res
    pub fn new(service: T) -> Self {
      Self {
        service,
        _t: PhantomData,
      }  
  }
}

impl<Arg: FromRequest, S> Extract<Arg, S> {
  pub fn new(service: S) -> Self {
    Self {
      service,
      _t: PhantomData,
    }
  }  
}
// Trait Implementation

impl<T, Arg, Res, O> Service for Wrapper<T, Arg, Res, O> 
where 
  T: Factory<Arg, Res, O>,
  Res: Future<Output=O>,
  O: Responder,
{
  type Request = Arg;
  type Response = String;
  type Error = ();
  // type Future = Ready<Result<Self::Response, ()>>;
  
  fn call(&mut self, param: Self::Request) -> Self::Response {
    let t = self.service.factory_call(param);
    let r = task::block_on(t);
    r.respond()
  }
}

impl<Arg: FromRequest, S> Service for ExtractService<Arg, S>
where
    S: Service<
            Request = Arg,
            Response = String,
        > + Clone,
{
    type Request = DB;
    type Response = String;
    type Error = ();

    fn call(&mut self, req: Self::Request) -> Self::Response {
      let t = Arg::from_request(req.clone());
      let b = self.service.call(t);
      b
    }
}


impl<Arg: FromRequest, S> ServiceFactory for Extract<Arg, S> 
where S: Service<
          Request = Arg,
          Response = String,
        > + Clone,
{
    type Request = DB;
    type Response = String;
    type Service = ExtractService<Arg, S>;
    type Error = ();

    fn new_service(&self) -> Self::Service {
      ExtractService {
        service: self.service.clone(),
        _t: PhantomData,
      }
    }
}


impl<T> Service for RouteServiceWrapper<T>
where
    T: Service<
        Request = DB,
        Response = String,
    >,
{
    type Request = DB;
    type Response = String;
    type Error = ();

    fn call(&mut self, req: Self::Request) -> Self::Response {
      let service = &mut self.service;
      service.call(req)
    }
}

// impl Service for BoxedRouteService {
//     type Request = DB;
//     type Response = String;
//     type Error = ();

//     fn call(&self, param: Self::Request) -> Self::Response {
//       (**self).call(param.clone())
//     }
// }

impl<T> RouteServiceFactory<T>
where
  T: ServiceFactory<
    Request=DB,
    Response=String,
  >,
  T::Service: 'static,
{
  pub fn new(service: T) -> Self {
    Self {
      service,
    }
  } 
}


impl<T> ServiceFactory for RouteServiceFactory<T> 
where 
  T: ServiceFactory<
    Request=DB,
    Response=String,
  >,
  T::Service: Service + 'static,
{
    type Request = DB;

    type Response = String;

    type Service = BoxedRouteService;
    type Error = ();

    fn new_service(&self) -> Self::Service {
      let service = &self.service;
      let service = service.new_service();
      Box::new(RouteServiceWrapper {
        service,
      })
    }
}