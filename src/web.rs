pub fn get<T>(route: &str, get:T) -> (&str, T) where T: Fn() {
    (route, get)
}