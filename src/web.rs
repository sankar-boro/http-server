use futures::Future;
use crate::service::{Factory};
use crate::responder::Responder;

pub fn get<T, R, O>(route: &str,  get:T) -> (&str, T) 
where 
  T: Factory<R, O> + 'static, 
  R: Future<Output=O>, 
  O: Responder 
{
  (route, get)
}