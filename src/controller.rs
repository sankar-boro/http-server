use crate::DB;
use crate::web;
use uuid::Uuid;
use serde::{Serialize};
use scylla::QueryResult;
use derive_more::{Display};
use scylla::{IntoTypedRows};
use std::fmt::{Error, Write};
use scylla::macros::FromRow;
use scylla::transport::errors::QueryError;
use scylla::frame::response::cql_to_rust::FromRow;


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

pub async fn get_user(app: web::Data<DB>, params: String) -> String {
    let _params: Vec<&str> = params.split("=").collect();
    if _params.len() != 2 {
        return "User not found".to_string();
    }
    let session = app.0.session.as_ref();
    let mut data = String::from("");
    if let Some(rows) = session.query(format!("SELECT userid, fname, lname, email FROM sankar.userCredentials WHERE email='{}'", _params[1].clone()), &[]).await.unwrap().rows {
        // Parse each row as a tuple containing single i32
        for row in rows.into_typed::<(Uuid,String,String,String)>() {
            let read_row: (Uuid,String,String,String) = row.unwrap();
            data.push_str(&read_row.0.to_string());
            data.push_str(" ");
            data.push_str(&read_row.1);
            data.push_str(" ");
            data.push_str(&read_row.2);
            data.push_str(" ");
            data.push_str(&read_row.3);
            data.push_str("\n");
        }
    }
    data
}

#[derive(Serialize, FromRow)]
#[allow(non_snake_case)]
pub struct UserResponse {
    userId: Uuid,
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

pub async fn get_all(app: web::Data<DB>) -> String {
    let session = app.0.session.as_ref();
    let a = session.query("SELECT userId, email, password, fname, lname from sankar.userCredentials", &[]).await.unwrap();
    let b = a.rows.map(|rows| {
        rows.into_typed::<UserResponse>()
            .map(|a| a.unwrap())
            .collect::<Vec<UserResponse>>()
    }).unwrap();
    let b: Vec<User> = b.iter().map(|_a| {
        User {
            fname: _a.fname.clone(),
            lname: _a.fname.clone(),
            email: _a.email.clone(),
            userId: _a.userId.to_string(),
        }
    }).collect();
    let t = serde_json::to_string(&b).unwrap();
    t
}
