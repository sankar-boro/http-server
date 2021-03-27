use crate::extensions::Extensions;

pub trait HttpRequest {}
pub trait DataFactory{
  fn set(&self, ext: &mut Extensions);
}

pub struct Request<T> {
  pub request: T
}

impl<T> Request<T> {
  pub fn new(request: T) -> Self {
    Request {
      request
    }
  }
}

impl<T> HttpRequest for Request<T> {

}

impl<T: Clone + 'static> DataFactory for Request<T> {
  fn set(&self, ext: &mut Extensions) {
    ext.insert(self.request.clone());
  }
}