use crate::DB;
use crate::web;
use std::fmt::{Error, Write};
use scylla::{IntoTypedRows, Session, SessionBuilder};


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

pub async fn delete_user(app: web::Data<DB>) -> String {
    let mut buf = String::new();
    // writer(&mut buf, &data).unwrap();
    // writer(&mut buf, "DELETE USER").unwrap();
    buf
}