fn main() {
    let mut data = Vec::new();
    let sample = String::from("ab:123+234-345*456///+hi100");
    let mut num_str = String::new(); 
    for c in sample.chars() {
        if c < '0' || c > '9' {
            if num_str.len() > 0 {
                data.push(num_str.clone());
                num_str.clear();
            }
            data.push(c.to_string());
        }else {
            num_str.push(c);
        }
    }

    if num_str.len() > 0 {
        data.push(num_str.clone());
    }
    println!("{:?}", data);
}