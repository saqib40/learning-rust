pub fn ex1() {
    println!("{}",largets_i32(2,3));
    println!("{}",largets_char('a','b'));
    println!("{}",largest(2,3));
}
fn largets_i32(a: i32, b:i32) -> i32 {
    if a>b {
        return a;
    }
    return b;
}
fn largets_char(a: char, b:char) -> char {
    if a>b {
        return a;
    }
    return b;
}
fn largest<T: std::cmp::PartialOrd>(a:T, b:T) -> T {
    if a>b {
        return a;
    }
    return b;
}