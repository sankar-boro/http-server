use std::fmt::{Error, Write};

fn writer<W: Write>(f: &mut W, s: &str) -> Result<(), Error> {
    f.write_fmt(format_args!("{}", s))
}

pub fn get_user(data: String) -> String {
    let mut buf = String::new();
    writer(&mut buf, "GET USER").unwrap();
    writer(&mut buf, &data).unwrap();
    buf
}

pub fn delete_user(data: String) -> String {
    let mut buf = String::new();
    writer(&mut buf, "DELETE USER").unwrap();
    writer(&mut buf, &data).unwrap();
    buf
}