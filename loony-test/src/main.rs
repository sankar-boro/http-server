use std::fmt::Write as FmtWrite;
use std::marker::PhantomData;

trait Service {
    type Request; 
    type Response;
    fn do_something(&self, p: Self::Request) -> Self::Response;
}
trait Factory<P, R>: Service {
    fn call(&self, param: P) -> R;
}

struct FactoryWrapper<T, P, R> {
    factory: T,
    _t: PhantomData<(P, R)>
}

impl<T, P, R> FactoryWrapper<T, P, R> {
    fn new(factory: T) -> Self {
        Self {
            factory,
            _t: PhantomData,
        }
    }
}

impl<T, P, R> Service for FactoryWrapper<T, P, R> where T: Factory<P, R> {
    type Request = P;
    type Response = R;

    fn do_something(&self, p: P) -> R {
        let factory = &self.factory;
        factory.call(p)
    }
}

trait Display {}
impl Display for String {}

impl<T> Service for T where T: Fn(String) -> String {
    type Request = String;
    type Response = String;
    fn do_something(&self, p: Self::Request) -> Self::Response {
        (self).call(p)
    }
}

impl<T: Service, P, R> Factory<P, R> for T where T: Fn(P) -> R {
    fn call(&self, param: P) -> R {
        (self)(param)
    }
}

fn index(param: String) -> String {
    let mut x = String::new();
    writeln!(&mut x, "{}, My name is Sankar.", param).unwrap();
    x
}

fn home(param: String) -> String {
    let mut x = String::new();
    writeln!(&mut x, "{}, You are at Home Page!", param).unwrap();
    x
}

fn profile(param: String) -> String {
    let mut x = String::new();
    writeln!(&mut x, "{}, You are Profile page", param).unwrap();
    x
}

type RouteData = Box<
    dyn Service<Request = String, Response = String>,
>;
struct Routes {
    routes: Vec<RouteData>,
}

fn run<T, P, R>(factory: T, routes: &mut Routes) -> &mut Routes
where 
    T: Factory<P, R> + 'static, 
    P: Display + 'static, 
    R: Display + 'static, 
{
    // routes.routes.push(Box::new(FactoryWrapper::new(factory)));
    routes
}

fn main() {
    let mut routes = Routes{
        routes: Vec::new(),
    };
    run(index, &mut routes);
    run(home, &mut routes);
    run(profile, &mut routes);

    for route in routes.routes.iter() {
        println!("{}", route.do_something("Hello World!".to_string()));
    }
}
