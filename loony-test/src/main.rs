#[allow(non_snake_case)]

mod responder;
mod request;
mod extensions;
mod array_test;

use responder::{HttpResponse, Responder};
use request::{HttpRequest, Request};

struct User {
  name: String,
}

fn index(request: Request<User>) -> HttpResponse {
  let mut data = String::from("Hello World!");
  data.push_str(" This is ");
  data.push_str(&request.request.name);
  HttpResponse::Ok().body(data)
}

fn home() -> impl Responder {
  "Hello World"
}

fn profile() -> impl Responder {
  String::from("Profile!")
}

fn route<T, A, R>(factory: T) where T: Fn(A) -> R, R: Responder, A: HttpRequest {

}

fn main() {
  let request = Request::new(User { name: "Sankar".to_owned() });
  let index = index(request);
  let response = index.get_body();
  println!("{}", response);
}