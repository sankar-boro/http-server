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
mod http;

use config::ServiceConfig;
use app::App;
use futures::future::ok;
use server::HttpServer;
use service::ServiceRequest;
use crate::responder::Responder;
use std::fmt::{Error, Write};
use scylla::{IntoTypedRows, Session, SessionBuilder};
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

async fn index(_: ()) -> impl Responder {
    String::from("Hello World")
}

fn routes(config: &mut ServiceConfig) {
    config.service(
        web::scope("/user")
        .route(route::get("/get/all").route(controller::get_user))
        .route(route::get("/get/{userName}").route(controller::get_user))
        .route(route::post("/delete").route(controller::delete_user))
    );

    config.service(
        web::scope("/test")
        .route(route::get("/scylla").route(controller::get_user))
        .route(route::post("/scylla/post").route(controller::get_user))
    );
}

#[derive(Debug, Clone)]
struct AppState {
    name: String,
}

#[derive(Clone)]
pub struct DB {
    session: Arc<Session>,
}
// pub trait FromRequest: Clone {
//   fn from_request(data: DB) -> Self;
// }

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
            name: "Loony".to_owned(),
        })
        .data(db.clone())
        .configure(routes)
        .route(web::get("/").route(index))
    )
    .run();
}   