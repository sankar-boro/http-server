use crate::DB;
use crate::app;
use scylla::FromRow;
use serde::Serialize;
use scylla::QueryResult;
use derive_more::Display;
use scylla::IntoTypedRows;
use std::fmt::{Error, Write};
use crate::service::HttpResponse;
use scylla::transport::errors::QueryError;


#[derive(Display, Debug)]
pub struct AppError {
    message: String,
}

#[derive(FromRow)]
pub struct GetUser {
    user_id: i32, 
    fname: String, 
    lname: String, 
    email: String
}

impl From<scylla::transport::errors::QueryError> for AppError {
    fn from(e: scylla::transport::errors::QueryError) -> Self {
        AppError {
            message: e.to_string(),
        }
    }
}

impl AppError {
    pub fn get_message(&self) -> &str {
        &self.message
    }
}

pub trait GetQueryResult<T> {
	type Request;
	fn get_query_result(self) -> Result<Option<Vec<Self::Request>>, AppError>;
}

impl<T: FromRow> GetQueryResult<T> for Result<QueryResult, QueryError> {
    type Request = T;
	fn get_query_result(self) -> Result<Option<Vec<Self::Request>>, AppError> {
		self
		.map_err(|err| AppError::from(err).into())
		.map(|res| {
			res.rows.map(|rows| {
				rows.into_typed::<Self::Request>()
					.map(|a| a.unwrap())
					.collect::<Vec<Self::Request>>()
			})
		})
    }
}

fn writer<W: Write>(f: &mut W, s: &str) -> Result<(), Error> {
    f.write_fmt(format_args!("{}", s))
}

pub async fn get_user(app: app::Data<DB>, params: String) -> String {
    let _params: Vec<&str> = params.split("=").collect();
    if _params.len() != 2 {
        return "User not found".to_string();
    }
    let conn = app.0.session.get().await.unwrap();
    let res = String::from("");
    let rows = conn.query(
        "SELECT * FROM users", 
        &[]
    ).await.unwrap();
    let uid: i32 = rows[0].get(0);
    println!("First user_id: {}", uid);
    res
}

#[derive(Serialize, FromRow)]
#[allow(non_snake_case)]
pub struct UserResponse {
    userId: i32,
	email: String,
	password: Vec<u8>,
	fname: String,
	lname: String,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct User {
    userId: String,
	email: String,
	fname: String,
	lname: String,
}

pub async fn get_all(app: app::Data<DB>) -> HttpResponse {

    let conn = app.0.session.get().await.unwrap();
    let res = String::from("");
    let rows = conn.query(
        "SELECT * FROM users", 
        &[]
    ).await.unwrap();
    let uid: i32 = rows[0].get(0);
    // format!("First user_id: {}", uid)
    // Ok(format!("First user_id: {}", uid))
    HttpResponse{value: format!("HttpResponse from value. User Id: {}", uid)}
}
