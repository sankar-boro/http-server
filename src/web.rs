use crate::service::{HttpServiceFactory};

pub struct FormData<T> {
  data: T,
}

pub fn get<T>(route: &str,  get:T) -> (&str, T) where T: HttpServiceFactory + 'static {
  (route, get)
}