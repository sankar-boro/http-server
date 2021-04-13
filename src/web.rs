use crate::{route::Route, scope::Scope};
use crate::route::Method;

fn method(method: Method) -> Route {
  Route::new().method(method)
}

pub fn get() -> Route
{
  method(Method::GET)
}

pub fn scope(scope: &str) -> Scope {
  Scope::new(scope)
}