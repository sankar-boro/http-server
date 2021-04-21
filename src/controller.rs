use std::fmt::{Error, Write};

fn writer<W: Write>(f: &mut W, s: &str) -> Result<(), Error> {
    f.write_fmt(format_args!("{}", s))
}

pub async fn get_user(data: String) -> String {
    let mut buf = String::new();
    writer(&mut buf, &data).unwrap();
    writer(&mut buf, "GET USER").unwrap();
    buf
}

pub async fn delete_user(data: String) -> String {
    let mut buf = String::new();
    writer(&mut buf, &data).unwrap();
    writer(&mut buf, "DELETE USER").unwrap();
    buf
}