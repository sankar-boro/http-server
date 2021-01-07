use crate::service::{HttpServiceFactory};

pub trait FormDataExtractor {}

pub struct FormData<T> {
  data: T,
}

impl<T> FormDataExtractor for FormData<T> {
  
}

pub fn get<T>(route: &str,  get:T) -> (&str, T) where T: HttpServiceFactory + 'static {
  (route, get)
}