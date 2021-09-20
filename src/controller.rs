use crate::DB;
use crate::app::App;
use crate::web;
use scylla::{IntoTypedRows};
use std::fmt::{Error, Write};
use scylla::frame::response::cql_to_rust::FromRow;
use scylla::macros::FromRow;
use serde::{Serialize, Deserialize};
use scylla::transport::errors::QueryError;
use scylla::QueryResult;
use derive_more::{Display};
use serde_json::json;
use uuid::Uuid;


#[derive(Display, Debug)]
pub struct AppError {
    message: String,
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

pub async fn get_user(app: web::Data<DB>) -> String {
    let session = app.0.session.as_ref();
    let mut data = String::new();
    if let Some(rows) = session.query("SELECT email FROM sankar.userCredentials", &[]).await.unwrap().rows {
        // Parse each row as a tuple containing single i32
        for row in rows.into_typed::<(String,)>() {
            let read_row: (String,) = row.unwrap();
            data.push_str(&read_row.0);
            data.push_str("\n");
        }
    }
    data
}

#[derive(Serialize, FromRow)]
pub struct UserResponse {
    userId: Uuid,
	email: String,
	password: Vec<u8>,
	fname: String,
	lname: String,
}

pub async fn get_all(app: web::Data<DB>) -> String {
    let session = app.0.session.as_ref();
    let a = session.query("SELECT userId, email, password, fname, lname from sankar.userCredentials", &[]).await.unwrap();
    let b = a.rows.map(|rows| {
        rows.into_typed::<UserResponse>()
            .map(|a| a.unwrap())
            .collect::<Vec<UserResponse>>()
    }).unwrap();
    let b = b.iter().map(|_a| {
        let u = json!({
            "name": format!("{} {}", _a.fname.clone(), _a.lname.clone()),
            "email": _a.email,
            "userId": _a.userId.to_string(),
        });
        u.to_string()			
	}).reduce(|a, b| {
            format!("{} {}", a, b)
        }).unwrap();
    
    b
}
