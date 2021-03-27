pub trait Responder {
  fn respond(self) -> HttpResponse;
}

pub trait StringModifier {
  fn respond(self) -> String;
}

impl StringModifier for String {
  fn respond(self) -> String {
    self
  }
}

impl StringModifier for &str {
  fn respond(self) -> String {
    self.to_owned()
  }
}

impl Responder for &str {
  fn respond(self) -> HttpResponse {
    HttpResponse {
      status: 200,
      body: self.to_owned(),
    }
  }
}

impl Responder for String {
  fn respond(self) -> HttpResponse {
    HttpResponse {
      status: 200,
      body: self,
    }
  }
}

pub struct HttpResponse {
  status: usize,
  body: String,
}

impl HttpResponse {
  pub fn Ok() -> Self {
    Self {
      status: 200,
      body: "".to_string(),
    }
  }

  pub fn body<T>(&self, body: T) -> Self where T: StringModifier {
    Self {
      status: self.status,
      body: body.respond(),
    }
  }

  pub fn get_body(&self) -> &str {
    &self.body
  }
}

/**
* Tests
*
*/


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
  }
}