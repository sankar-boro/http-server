use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut newfile = File::open("./file").unwrap();
    let mut new_buffer = Vec::<u8>::new();
    newfile.read_to_end(&mut new_buffer).unwrap();
    println!("{:?}", new_buffer);
}
