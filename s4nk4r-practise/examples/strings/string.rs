use regex::Regex;

fn main() {
    let reg = Regex::new(r"/user/get/[[:alnum:]]{2,5}-[[:alnum:]]{2}-[[:alnum:]]{2}").unwrap();
    assert!(reg.is_match("/user/get/9ssd-s0-0d"));
}