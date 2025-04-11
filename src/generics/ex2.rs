pub fn ex2() {
    println!("{}", add_integers(5, 10));
    println!("{}", add_floats(5.5, 2.5));
    println!("{}", add(5, 10));
    println!("{}", add(5.5, 2.5));
}

fn add_integers(a: i32, b: i32) -> i32 {
    a + b
}

fn add_floats(a: f64, b: f64) -> f64 {
    a + b
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T { // std::ops::Add ensures that T supports the + operator
    a + b
}