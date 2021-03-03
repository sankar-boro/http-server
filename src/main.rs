mod app;
mod responder;
mod server;
mod service;
mod route;
mod controller;
mod extensions;
mod web;

use service::ServiceConfig;
use app::App;
use loony_http::Response;
use responder::Responder;
use server::HttpServer;

pub struct Request;
struct User {
    name: String,
}

fn index(data: web::FormData<User>) -> impl Responder {
    let res = Response::from(String::from("Loony"));
    res
}

fn delete() -> impl Responder {
    let res = Response::from(32);
    res
}

fn routes(config: &mut ServiceConfig) {
    config.service(
        route::scope("/user")
        .route(
            web::get("/get", controller::get_user)
        ).route(
            web::get("/delete", controller::delete_user)
        )
    );
}

#[derive(Debug)]
struct AppState {
    name: String,
}

fn main() {
    HttpServer::new(move ||
        App::new()
        .app_data( AppState {
            name: "Loony".to_owned(),
        })
        .configure(routes)
        // .route("/", web::get("/get", controller::get_user))
    )
    .run();
}   
