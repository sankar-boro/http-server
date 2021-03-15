use std::convert::From;
use std::{fmt::Write as FmtWrite, io::Stdin};
use std::marker::PhantomData;

trait Service {
    type Request; 
    type Response;
    fn do_something(&self, p: Self::Request) -> Self::Response;
}

struct User {
    name: String,
}

pub struct Response {
    pub body: String,
    pub error: Option<String>,
}

// impl Response {
//     pub fn new(body: String) -> Self {

//         Self { body, error: None }
//     }
// }
impl From<String> for Response {
    fn from(body: String) -> Self {
        Self{
            body,
            error: None,
        }
    }
}

impl From<u8> for Response {
    fn from(body: u8) -> Self {
        Self{
            body: body.to_string(),
            error: None,
        }
    }
}


pub trait Responder {
    // type Error: Into<Error>;
    // fn respond(body: String) -> Result<Response, Error>;
    fn respond(&self) -> String;
}

impl Responder for Response {
    // fn respond(body: String) -> Result<Response, Error> {
    fn respond(&self) -> String {
        // Ok(Response { body, error: None })
        self.body.to_string()
    }
}


pub trait FormDataExtractor {}

pub struct FormData<T> {
  data: T,
}

impl<T> FormDataExtractor for FormData<T> {
  
}

// trait Factory<P, R> 
// where 
//     P: Display,
//     R: Display 
// {
//     fn call(&self, param: P) -> R;
// }

pub trait Factory<T, R>: Clone + 'static
where
    R: Responder
{
    fn call(&self, param: T) -> R;
}

impl<F, R> Factory<(), R> for F
where
    F: Fn() -> R + Clone + 'static,
    R: Responder
{
    fn call(&self, _: ()) -> R {
        (self)()
    }
}

// impl<P, R> Factory<P, R> for P
// where
//     P: Fn() -> R + Clone + 'static,
//     R: Display,
// {
//     fn call(&self, _: ()) -> R {
//         (self)()
//     }
// }
trait ServiceFactory {
    // type Request;
    // type Response;
    type Service;
}

struct FactoryWrapper<T, P, R> {
    factory: T,
    _t: PhantomData<(P, R)>
}

impl<T, P, R> FactoryWrapper<T, P, R>  
where 
    T: Factory<P, R>,
    P: FormDataExtractor,
    R: Responder,
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
    P: FormDataExtractor,
    R: Responder,
{
    type Request = P;
    type Response = R;

    fn do_something(&self, p: Self::Request) -> Self::Response {
        let factory = &self.factory;
        factory.call(p)
    }
}

// trait Display {
//     fn display(self) -> String;
// }

// impl Display for String {
//     fn display(self) -> String {
//         self
//     }
// }

// trait DisplayString {
//     fn display(self) -> String;
// }

// impl DisplayString for String {
//     fn display(self) -> String {
//         self
//     }
// }

impl<T, P, R> Factory<P, R> for T 
where 
    T: Fn(P) -> R + Clone + 'static, 
    R: Responder, 
    P: FormDataExtractor 
{
    fn call(&self, param: P) -> R {
        (self)(param)
    }
}

fn index(param: FormData<User>) -> impl Responder {
    let mut x = String::new();
    writeln!(&mut x, "{}, My name is Sankar.", param.data.name).unwrap();
    Response::from(x)
}

fn home(param: FormData<User>) -> impl Responder {
    let mut x = String::new();
    writeln!(&mut x, "{}, You are at Home Page!", param.data.name).unwrap();
    Response::from(x)
}

fn profile(param: FormData<User>) -> impl Responder {
    let mut x = String::new();
    writeln!(&mut x, "{}, You are Profile page", param.data.name).unwrap();
    Response::from(x)
}

type RouteData = Box<
    dyn Service<Request = String, Response = String>,
>;

type AllRouteData = Box<
    // dyn ServiceFactory<Request = String, Response = String, Service= RouteData>,
    // dyn ServiceFactory<Request = String, Response = String>,
    dyn ServiceFactory<Service= RouteData>,
>;

struct Routes {
    routes: Vec<RouteData>,
}


struct Extract<T: FormDataExtractor, S> {
    service: S,
    _t: PhantomData<T>,
}

impl<T: FormDataExtractor, S> Extract<T, S> {
    pub fn new(service: S) -> Self {
        Extract {
            service,
            _t: PhantomData,
        }
    }
}

struct RouteNewService<T>
where
    T: ServiceFactory,
    T::Service: Service,
{
    service: T,
}

impl<T> RouteNewService<T>
where
    T: ServiceFactory,
    T::Service: Service + 'static,
{
    pub fn new(service: T) -> Self {
        RouteNewService { service }
    }
}

impl<T> ServiceFactory for RouteNewService<T>
where
    T: ServiceFactory,
    T::Service: Service + 'static,
{
    type Service = RouteData;
}

impl<T: FormDataExtractor, S> ServiceFactory for Extract<T, S>
where
    S: Service
{
    type Service = RouteData;
}

fn run<T, P, R>(factory: T, routes: &mut Routes) -> &mut Routes
where 
    T: Factory<P, R> + 'static, 
    P: FormDataExtractor + 'static, 
    R: Responder + 'static, 
{

    // ----------------------------------------------------------
    // let name = "".to_string();
    // let one = FactoryWrapper::new(|name: String| {
    //     "Hello Bro".to_string()
    // });
    // let two = Extract::new(one);
    // ----------------------------------------------------------

    // let y =Service+ Extract::new(x);
    // let result = FactoryWrapper::new(factory);
    // let result = Box::new(FactoryWrapper::new(factory));
    // routes.routes.push(Box::new(x));
    // ----------------------------------------------------------
    
    routes
}

fn main() {
    // let mut routes = Routes{
    //     routes: Vec::new(),
    // };
    // run(index, &mut routes);
    // run(home, &mut routes);
    // run(profile, &mut routes);

    // for route in routes.routes.iter() {
    //     println!("{}", route.do_something("Hello World!".to_string()));
    // }
}

/// FromRequest trait impl for tuples
macro_rules! factory_tuple ({ $(($n:tt, $T:ident)),+} => {
    impl<Func, $($T,)+ Res> Factory<($($T,)+), Res> for Func
    where Func: Fn($($T,)+) -> Res + Clone + 'static,
          Res: Responder,
    {
        fn call(&self, param: ($($T,)+)) -> Res {
            (self)($(param.$n,)+)
        }
    }
});

#[rustfmt::skip]
mod m {
    use super::*;

    factory_tuple!((0, FormDataExtractor));
//   factory_tuple!((0, FormDataExtractor), (1, String));
  // factory_tuple!((0, A), (1, B), (2, C));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I));
  // factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J));
}