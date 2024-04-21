fn main() {
    let mut data = String::from("sankar");
    let x = &data;
    let y = &mut data;
    *y = String::from("boro");

    println!("{}", data)
}