use std::fmt::Write as FmtWrite;
use std::marker::PhantomData;

trait Service {
    type Request; 
    type Response;
    fn do_something(&self, p: Self::Request) -> Self::Response;
}
trait Factory<P, R> 
where 
    P: Display,
    R: Display 
{
    fn call(&self, param: P) -> R;
}

struct FactoryWrapper<T, P, R> {
    factory: T,
    _t: PhantomData<(P, R)>
}

impl<T, P, R> FactoryWrapper<T, P, R>  
where 
    T: Factory<P, R>,
    P: Display,
    R: Display,
{
    fn new(factory: T) -> Self {
        Self {
            factory,
            _t: PhantomData,
        }
    }
}

impl<T, P, R> Service for FactoryWrapper<T, P, R> 
where 
    T: Factory<P, R>,
    P: Display,
    R: Display,
{
    type Request = P;
    type Response = String;

    fn do_something(&self, p: Self::Request) -> Self::Response {
        let factory = &self.factory;
        let x = factory.call(p);
        x.display()
    }
}

trait Display {
    fn display(self) -> String;
}

impl Display for String {
    fn display(self) -> String {
        self
    }
}

// impl<T> Service for T where T: Fn(String) -> String {
//     type Request = String;
//     type Response = String;
//     fn do_something(&self, p: Self::Request) -> Self::Response {
//         (self).call(p)
//     }
// }

impl<T, P, R> Factory<P, R> for T where T: Fn(P) -> R, R: Display, P: Display {
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

trait ExtractService {}
struct Extract<T: Display, S: Service> {
    service: S,
    _t: PhantomData<T>,
}

impl<T: Display, S: Service> Extract<T, S> {
    pub fn new(service: S) -> Self {
        Extract {
            service,
            _t: PhantomData,
        }
    }
}

impl<T, S> ExtractService for Extract<T, S> 
where 
    T: Display,
    S: Service,
{

}

fn run<T, P, R>(factory: T, routes: &mut Routes) -> &mut Routes
where 
    T: Factory<P, R> + 'static, 
    P: Display + 'static, 
    R: Display + 'static, 
{

    // ----------------------------------------------------------
    // let name = "".to_string();
    let x = FactoryWrapper::new(|name: String| {
        "Hello Bro".to_string()
    });
    // let y = Extract::new(x);
    // let result = FactoryWrapper::new(factory);
    // let result = Box::new(FactoryWrapper::new(factory));
    routes.routes.push(Box::new(x));
    // ----------------------------------------------------------
    
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
