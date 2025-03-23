fn append_message(msg: &mut String) {
    msg.push_str(" - Learning Rust Ownership");
}

pub fn mutable() {
    let mut s = String::from("Hello");
    append_message(&mut s); // Mutable borrow
    println!("Updated: {}", s);
}
