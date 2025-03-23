pub fn rules() {
    let mut s = String::from("Rust");
    let r1 = &s; // Immutable borrow
    let r2 = &s; // Immutable borrow
    // let r3 = &mut s; // Error: Mutable borrow not allowed
    println!("{} {}", r1, r2);
}
