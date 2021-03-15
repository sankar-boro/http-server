use futures::Future;
use crate::service::{Factory};
use crate::responder::Responder;

pub fn get<T, P, R, O>(route: &str,  get:T) -> (&str, T) 
where 
  T: Factory<P, R, O> + 'static, 
  P: FormDataExtractor,
  R: Future<Output=O>, 
  O: Responder 
{
  (route, get)
}

pub trait FormDataExtractor {}

pub struct FormData {
  data: String,
}

impl FormDataExtractor for FormData {
  
}
