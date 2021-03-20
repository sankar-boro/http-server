
use crate::{FromRequest, service::{Factory}};
use crate::responder::Responder;

pub fn get<T, P, R, O>(route: &str,  get:T) -> (&str, T) 
where 
  T: Factory<P, R> + Clone + 'static, 
  P: FromRequest,
  // R: Future<Output=O>, 
  R: Responder
{
  (route, get)
}