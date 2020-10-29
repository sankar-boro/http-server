trait HttpServiceFactory {
    fn service(&self) -> String;
}
trait Responder {
    fn respond(&self) -> String;
}
struct Response {
    data: String,
}
impl Responder for Response {
    fn respond(&self) -> String {
        self.data.to_string()
    }
}
fn index() -> impl Responder {
    Response {
        data: "Hello".to_string()
    }
}
fn delete() -> impl Responder {
    Response {
        data: "Hello".to_string()
    }
}
// impl<T, R> HttpServiceFactory for T where T: Fn() -> R, R:Responder {
//     fn service(&self) -> String {
//         let som = (self)();
//         som.respond()
//     }
// }
struct App{
    services: Vec<HttpResponse>
}

impl App {
    fn new() -> Self {
        Self {
            services:Vec::new(),
        }
    }
    fn route(mut self, path: &str, res:HttpResponse) -> Self {
        self.services.push(res);
        self
    }
}
impl HttpServiceFactory for Responder {
    fn service(&self) -> String {
        "".to_owned()
    }
}
struct HttpResponse;
impl HttpResponse {
    fn Ok() -> Self {
        Self
    }
    fn body(&self) -> Self{ 
        Self
    }
}
fn get<T,A>(f:T) -> HttpResponse where T: Fn() -> A, A: Responder {
    HttpResponse::Ok().body()
}
fn main() {
    App::new().route("/home", get(index)).route("/delete", get(delete));
}