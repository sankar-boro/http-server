use loony_futures::{new_channel, Sankar, Hooker, Sex};
use std::time::Duration;
fn main() {
    let (sankar, hooker) = new_channel();
    hooker.hook_new_girl(async {
        println!("sankar");
        Sex::duration(Duration::new(2, 0)).await;
        println!("boro");
    });
    hooker.hook_new_girl(async {
        println!("arun");
        Sex::duration(Duration::new(5, 0)).await;
        println!("kachari");
    });
    sankar.have_sex();
}
