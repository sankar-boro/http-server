use crate::service::{HttpServiceFactory, Factory};
use crate::responder::Responder;

pub trait FormDataExtractor {}

pub struct FormData<T> {
  data: T,
}

impl<T> FormDataExtractor for FormData<T> {
  
}

pub fn get<F, I, R>(route: &str,  get:F) -> (&str, F) where F: Factory<I, R>, R: Responder {
  (route, get)
}


// pub fn to<F, I, R, U>(handler: F) -> Route
// where
//     F: Factory<I, R, U>,
//     I: FromRequest + 'static,
//     R: Future<Output = U> + 'static,
//     U: Responder + 'static,
// {
//     Route::new().to(handler)
// }