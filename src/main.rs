#![allow(dead_code)]

mod app;
mod responder;
mod server;
mod service;
mod route;
mod controller;
mod extensions;
mod web;
mod builder;
mod scope;

use service::ServiceConfig;
use app::App;
use server::HttpServer;
use crate::responder::Responder;
use std::fmt::{Error, Write};

#[derive(Debug)]
pub struct Request {
    method: String,
    version: String,
    url: String,
}

fn writer<W: Write>(f: &mut W, s: &str) -> Result<(), Error> {
    f.write_fmt(format_args!("{}", s))
}

async fn index(data: String) -> impl Responder {
    let mut buf = String::new();
    writer(&mut buf, "Hello World! ").unwrap();
    writer(&mut buf, &data).unwrap();
    buf
}

fn routes(config: &mut ServiceConfig) {
    config.service(
        route::scope("/user")
        .route("/get", route::get(controller::get_user))
        .route("/delete", route::post(controller::delete_user))
    );
}

#[derive(Debug, Clone)]
struct AppState {
    name: String,
}
pub trait FromRequest: Clone {
  fn from_request(data: String) -> Self;
}

impl FromRequest for String {
    fn from_request(data: String) -> Self {
      data
    }
}


#[async_std::main]
async fn main() {
    HttpServer::new(move ||
        App::new()
        .app_data( AppState {
            name: "Loony".to_owned(),
        })
        .configure(routes)
        .route(("/", index))
    )
    .run();
}   
