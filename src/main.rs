#![allow(dead_code)]

mod app;
mod responder;
mod server;
mod handler;
mod route;
mod controller;
mod extensions;
mod web;
mod builder;
mod scope;
mod resource;
mod config;
mod default;
mod connection;
mod response;
mod request;
mod service;
mod extract;
mod app_service;

use config::ServiceConfig;
use app::App;
use server::HttpServer;
use service::ServiceRequest;
use crate::responder::Responder;
use std::fmt::{Error, Write};
use scylla::{ Session, SessionBuilder};
use std::sync::Arc;
use std::future::{Ready, ready};
use extract::FromRequest;

#[derive(Debug)]
pub struct Request {
    method: String,
    version: String,
    url: String,
}

fn writer<W: Write>(f: &mut W, s: &str) -> Result<(), Error> {
    f.write_fmt(format_args!("{}", s))
}

async fn index() -> impl Responder {
    String::from("Hello World")
}

fn routes(config: &mut ServiceConfig) {
    config.service(
        web::scope("/user")
        .route(route::get("/get::userid").route(controller::get_user)) // expects an url of /user/get/one?userid=<somevalue>
        .route(route::get("/delete::userid").route(controller::get_user)) // expects an url of /user/get/one?userid=<somevalue>
        .route(route::get("/update::userid").route(controller::get_user)) // expects an url of /user/get/one?userid=<somevalue>
    );
}

#[derive(Debug, Clone)]
pub struct AppState {
    name: String,
}

#[derive(Clone)]
pub struct DB {
    session: Arc<Session>,
}

impl FromRequest for web::Data<DB> {
    type Future = Ready<Result<web::Data<DB>, ()>>;
    fn from_request(req: &ServiceRequest) -> Self::Future {
        let a = req.0.extensions.get::<DB>().unwrap();
        ready(Ok(web::Data(a.clone())))
    }
}

impl FromRequest for (web::Data<DB>, ) {
    type Future = Ready<Result<(web::Data<DB>, ), ()>>;
    fn from_request(req: &ServiceRequest) -> Self::Future {
        let a = req.0.extensions.get::<DB>().unwrap();
        ready(Ok((web::Data(a.clone()), )))
    }
}

impl FromRequest for () {
    type Future = Ready<Result<(), ()>>;
    fn from_request(_: &ServiceRequest) -> Self::Future {
        ready(Ok(()))
    }
}

impl FromRequest for (String, ) {
    type Future = Ready<Result<(String,), ()>>;
    fn from_request(_: &ServiceRequest) -> Self::Future {
        ready(Ok(("".to_string(), )))
    }
}


#[tokio::main]
async fn main() {
    let uri = std::env::var("SCYLLA_URI")
        .unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    let session: Session = SessionBuilder::new()
        .known_node(uri)
        .build()
        .await.unwrap();

    let db = DB {
        session: Arc::new(session),
    };

    HttpServer::new(move ||
        App::new()
        .app_data( AppState {
            name: "s4nk4r".to_owned(),
        })
        .data(db.clone())
        .configure(routes)
        .route(web::get("/").route(index))
    )
    .run();
}   