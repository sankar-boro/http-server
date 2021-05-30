use crate::{route::Route, scope::Scope};
use crate::route::Method;

fn method(path: &str, method: Method) -> Route {
  Route::new(path).method(method)
}

pub fn get(path: &str) -> Route
{
  method(path, Method::GET)
}

pub fn scope(scope: &str) -> Scope {
  Scope::new(scope)
}

#[derive(Clone)]
pub struct Data<T>(pub T);