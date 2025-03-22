// an example of variables
pub fn variables() {
    let x = 10;       // Immutable => different from JS, cause you can still update the contents of const arrays and objects in JS
    let mut y = 20;   // Mutable
    const PI: f64 = 3.14159;
    println!("x: {}, y: {}, PI: {}", x, y, PI);
    y = 25;
    println!("Updated y: {}", y);
}
