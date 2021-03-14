trait Describe{}

type FnArg = Box<dyn Fn() -> Describe>;

struct User {
    describe: FnArg,
}

fn main() {
    let user = User{
        describe: Box::new(|| {
            return "Sankar".to_string()
        }),
    };

    // Short cut
    // let user_name = (user.describe)();
    
    // Long Cut
    // let user = user.describe;
    // let user_name = user();
}