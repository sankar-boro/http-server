use crate::service::{HttpServiceFactory};

pub fn get<T>(route: &str, get:T) -> (&str, T) where T: HttpServiceFactory + 'static {
  (route, get)
}