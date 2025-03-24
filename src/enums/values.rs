enum Shape {
    Circle(f64),  // Variant with associated data (radius)
    Square(f64),  // Variant with associated data (side length)
    Rectangle(f64, f64),  // Variant with associated data (width, height)
}

enum Message {
    Text(String),
    Coordinates(i32, i32),
    Quit,
}

fn process_message(msg: Message) {
    match msg {
        Message::Text(text) => println!("Message: {}", text),
        Message::Coordinates(x, y) => println!("Location: ({}, {})", x, y),
        Message::Quit => println!("Quitting..."),
    }
}

pub fn value() {
    let msg1 = Message::Text(String::from("Hello, Rust!"));
    let msg2 = Message::Coordinates(10, 20);
    let msg3 = Message::Quit;

    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
}
