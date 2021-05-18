use std::slice::Iter;

static data: &'static [u8; 20] = b"GET /home HTTP/1.1\r\n";
enum RType {
    Method,
    Uri,
}
fn main() {
    let buffer = data.clone();
    let mut buffer = buffer.iter();
    let mut index: usize = 0;
    let method = get_method(&mut buffer, &mut index);
    index += 1;
    let uri = get_uri(&mut buffer, &mut index);
    index += 1;
    let version = get_version(&mut buffer, &mut index);
    println!("{:?}", method);
    println!("{:?}", uri);
    println!("{:?}", version);
}

fn get_method(buffer: &mut Iter<u8>, index:&mut usize) -> Option<String> {
    let mut pointer= index.clone();
    loop {
        match buffer.next() {
            Some(b) => {
                if *b == b' ' {
                    let rdata = Some(String::from_utf8_lossy(&data[*index..pointer]).to_string());
                    *index = pointer;
                    return rdata;
                }
                pointer += 1;
            }
            None => {
                return None;
            }
        }
    }
}

fn get_uri(buffer: &mut Iter<u8>, index:&mut usize) -> Option<String> {
    let mut pointer= index.clone();
    loop {
        match buffer.next() {
            Some(b) => {
                if *b == b' ' {
                    let rdata = Some(String::from_utf8_lossy(&data[*index..pointer]).to_string());
                    *index = pointer;
                    return rdata;
                }
                pointer += 1;
            }
            None => {
                return None;
            }
        }
    }
}

fn get_version(buffer: &mut Iter<u8>, index:&mut usize) -> Option<String> {
    let mut pointer= index.clone();
    loop {
        match buffer.next() {
            Some(b) => {
                if *b == b'\r' {
                    let rdata = Some(String::from_utf8_lossy(&data[*index..pointer]).to_string());
                    *index = pointer;
                    return rdata;
                }
                pointer += 1;
            }
            None => {
                return None;
            }
        }
    }
}