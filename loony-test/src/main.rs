trait Responder {}
struct HttpResponse(String);
impl Responder for String {}
impl Responder for HttpResponse {}

fn index() -> impl Responder {
  String::from("Index")
}

fn home() -> impl Responder {
  String::from("Home")
}

struct App<T>{
  services: Vec<Box<T>>
}

fn run<T, R>(factory: T) where T: Fn() -> R, R: Responder {
  let mut services = Vec::new();
  services.push(Box::new(factory));
  let app = App {
    services,
  };
}

fn main() {
  run(index);
}