fn display_message(msg: &String) {
    println!("Message: {}", msg);
}

pub fn immutable() {
    let s = String::from("Hello, Rust!");
    display_message(&s); // Immutable borrow
    println!("Still available: {}", s);
}
