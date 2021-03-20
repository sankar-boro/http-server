#![allow(dead_code)]
#[macro_export]
macro_rules! ping {
  () => {
    println!("pong");
  };
}

macro_rules! create_function {
  ($fn_name: ident) => {
    fn $fn_name() {
      println!("You called function: {:?}", stringify!($fn_name));
    }
  };
}

#[macro_export]
macro_rules! print_bodmas_cal {
    ($expression:expr) => {
      println!(
        "{:?} = {:?}",
        stringify!($expression),
				$expression
			);
    };
}

#[macro_export]
macro_rules! sum_of {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        $x + sum_of!($($y),+)
    )
}

create_function!(ping);