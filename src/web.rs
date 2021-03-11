use futures::Future;
use crate::service::{Factory};

pub fn get<F, I, R>(route: &str,  get:F) -> (&str, F) where F: Factory<I, R> + 'static, R: Future<Output=String> {
  (route, get)
}