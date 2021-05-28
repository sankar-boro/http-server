use regex::Regex;

fn main() {
    let reg = Regex::new(r"^/user/get/[a-z0-9]$").unwrap();
    assert!(reg.is_match("/user/get/sdf98"));
}