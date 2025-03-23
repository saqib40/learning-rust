struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to calculate area
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Associated function (like static method)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

pub fn implement() {
    let rect = Rectangle::new(10, 20);
    println!("Area: {}", rect.area());
}
