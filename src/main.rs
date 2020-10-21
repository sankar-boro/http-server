mod app;
mod responder;
mod server;
mod service;
mod route;
mod controller;
mod extensions;

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
            route::get("/get", controller::get_user)
        ).route(
            route::get("/delete", controller::delete_user)
        )
    );
}
#[derive(Debug)]
struct AppState {
    name: String,
}
fn main() {
    HttpServer::new(|| 
        App::new()
        .app_data( AppState {
            name: "Loony".to_owned(),
        })
        .service(routes)
        .route("/", index)
        .route("/delete", delete)
    )
    .run();
}
