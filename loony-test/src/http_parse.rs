use std::slice::Iter;

static DATA: &'static [u8; 20] = b"GET /home HTTP/1.1\r\n";

fn main() {
    let buffer = DATA.clone();
    let buffer_ref = &buffer;
    let mut iter_buffer = buffer_ref.iter();

    let mut index:usize = 0;

    let method = head(&mut iter_buffer, &mut index);
    let uri = head(&mut iter_buffer, &mut index);
    let version = get_version(&mut iter_buffer, &mut index);

    if let Some((mh, mt)) = method {
        println!("{}", String::from_utf8_lossy(&buffer_ref[mh..mt]).to_string());
    }
    if let Some((uh, ut)) = uri {
        println!("{}", String::from_utf8_lossy(&buffer_ref[uh..ut]).to_string());
    }
    if let Some((vh, vt)) = version {
        println!("{}", String::from_utf8_lossy(&buffer_ref[vh..vt]).to_string());
    }
}

fn head(buffer: &mut Iter<u8>, index: &mut usize) -> Option<(usize, usize)> {
    let mut count: usize = 0;
    let start = *index;

    loop {
        match buffer.next() {
            Some(b) => {
                if *b == b' ' {
                    let rd = (start, start + count);
                    *index = start + count + 1;
                    return Some(rd);
                }
                count += 1;
            }
            None => {
                return None;
            }
        }
    }
}

fn get_version(buffer: &mut Iter<u8>, index: &mut usize) -> Option<(usize, usize)> {
    let mut count: usize = 0;
    let start = *index;

    loop {
        match buffer.next() {
            Some(b) => {
                if *b == b'\n' {
                    let rd = (start, start + count);
                    *index = start + count + 1;
                    return Some(rd);
                }
                count += 1;
            }
            None => {
                return None;
            }
        }
    }
}