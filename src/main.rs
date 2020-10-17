mod app;
mod responder;
mod server;
mod service;
mod route;
mod controller;

use service::ServiceConfig;
use app::App;
use loony_http::Response;
use responder::Responder;
use server::HttpServer;

fn index() -> impl Responder {
    let res = Response::from(String::from("Loony"));
    res
}

fn delete() -> impl Responder {
    let res = Response::from(32);
    res
}

fn routes(config: &mut ServiceConfig) {
    config.service(
        route::scope("/user").route(
            // let data = controller::get_user;
            // route::get("/get", controller::get_user)
            route::get("/get", controller::get_user)
        ).route(
            route::get("/delete", controller::delete_user)
        )
    );
}
fn main() {
    HttpServer::new(|| App::new().config(routes).route("/", index).route("/delete", delete)).run();
}
