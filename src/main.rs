#![allow(dead_code)]

mod app;
mod responder;
mod server;
mod service;
mod route;
mod controller;
mod extensions;
mod web;
mod extract;

use service::ServiceConfig;
use app::App;
use loony_http::Response;
use server::HttpServer;

#[derive(Debug)]
pub struct Request {
    method: String,
    version: String,
    url: String,
}

async fn index(data: web::FormData) -> Response {
    // println!("{:?}", request);
    Response::ok("Hello World".to_string())
}

fn routes(config: &mut ServiceConfig) {
    config.service(
        route::scope("/user").route("/get", controller::get_user).route("/delete", controller::delete_user)
    );
}

#[derive(Debug)]
struct AppState {
    name: String,
}


#[async_std::main]
async fn main() {
    HttpServer::new(move ||
        App::new()
        .app_data( AppState {
            name: "Loony".to_owned(),
        })
        .configure(routes)
        .route(web::get("/", index))
    )
    .run();
}   
