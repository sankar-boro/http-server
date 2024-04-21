#![allow(dead_code)]
#[allow(unused_imports)]
mod app;
mod responder;
mod server;
mod handler;
mod route;
mod controller;
mod extensions;
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
mod http_service;

use connection::pg_connection;
use config::ServiceConfig;
use app::App;
use deadpool_postgres::Pool;
use server::HttpServer;
use service::ServiceRequest;
use crate::responder::Responder;
use std::fmt::{Error, Write};
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
        route::scope("/user")
        .route(route::get("/all").to(controller::get_all))
        .route(route::get("/get::userid").to(controller::get_user)) // expects an url of /user/get/one?userid=<somevalue>
        .route(route::get("/delete::userid").to(controller::get_user)) // expects an url of /user/get/one?userid=<somevalue>
        .route(route::get("/update::userid").to(controller::get_user)) // expects an url of /user/get/one?userid=<somevalue>
    );
}

#[derive(Debug, Clone)]
pub struct AppState {
    name: String,
}

#[derive(Clone)]
pub struct DB {
    pub session: Pool,
}

impl FromRequest for app::Data<DB> {
    type Future = Ready<Result<app::Data<DB>, ()>>;
    fn from_request(req: &ServiceRequest) -> Self::Future {
        let a = req.0.extensions.get::<DB>().unwrap();
        ready(Ok(app::Data(a.clone())))
    }
}

impl FromRequest for (app::Data<DB>, ) {
    type Future = Ready<Result<(app::Data<DB>, ), ()>>;
    fn from_request(req: &ServiceRequest) -> Self::Future {
        let a = req.0.extensions.get::<DB>().unwrap();
        ready(Ok((app::Data(a.clone()), )))
    }
}

impl FromRequest for (app::Data<DB>, String,) {
    type Future = Ready<Result<(app::Data<DB>, String,), ()>>;
    fn from_request(req: &ServiceRequest) -> Self::Future {
        let a = req.0.extensions.get::<DB>().unwrap();
        let b = &req.0.params;
        if let Some(b) = b {
            if b.len() == 1  {
                return ready(Ok((app::Data(a.clone()), b[0].clone(),)));
            }
        }

        ready(Ok((app::Data(a.clone()), "".to_string(),)))
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

    let conn = pg_connection().await;

    let db = DB {
        session: conn,
    };

    HttpServer::new(move ||
        App::new()
        .app_data( AppState {
            name: "loony".to_owned(),
        })
        .data(db.clone())
        .configure(routes)
        .route(route::get("/").to(index))
    )
    .run();

}   