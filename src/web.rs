use std::future::Future;

use crate::responder::Responder;
use crate::{FromRequest, service::{Factory}};

pub fn get<T, P, R, O>(route: &str,  get:T) -> (&str, T) 
where 
  T: Factory<P, R, O> + Clone + 'static, 
  P: FromRequest,
  R: Future<Output=O>, 
  O: Responder
{
  (route, get)
}