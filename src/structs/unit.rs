// Structs without fields.
// Useful for (marker traits or) implementing behavior without data
// Efficient because they don’t store any data.

struct Logger;

impl Logger {
    fn log(&self, message: &str) {
        println!("[LOG]: {}", message);
    }
}

pub fn unit() {
    let logger = Logger;
    logger.log("Rust is awesome!");
}
