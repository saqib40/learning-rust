// Similar to tuples, but with distinct types.
// Useful when field names are unnecessary.

struct Coordinates(f64, u8);

pub fn tuple() {
    let point = Coordinates(10.5, 3);
    println!("X: {}, Y: {}", point.0, point.1);
}
